using System.Diagnostics;
using InstrumentComponents.Drivers;
using InstrumentComponents.Drivers.Catalog;
using InstrumentComponents.Drivers.Keysight;
using InstrumentComponents.Runtime;
using InstrumentComponents.Scpi;

internal static class HpAsyncProof
{
    private static void Require(bool value, string message) { if (!value) throw new InvalidOperationException(message); }
    private static IDriverFactory Factory(int index, Host host)
    {
        if (index < 2) return new Hp5334Factory(index == 0 ? Hp5334Model.Hp5334A : Hp5334Model.Hp5334B, Hp5334Commissioning.NativeInternalGatingConfirmed, host, host);
        var endpoint = new Endpoint("hpib", "fixture");
        if (index == 2) return new Hp3497Factory([new(endpoint, new(true, true, [new(7, 0, 0, Hp3497Module.Option010, Hp3497Compensation.None)]))], host, host);
        var model = index switch { 3 => HpRadioModel.Hp8920A, 4 => HpRadioModel.Hp8920B, _ => HpRadioModel.Hp8921A };
        return new HpRadioFactory(model, [new(endpoint, new(model, index switch { 3 => "A.18.00", 4 => "B.06.00", _ => "A.15.00" }, HpRadioAttenuator.Electronic, HpRadioInputPower.Standard, HpRadioSourcePort.RfInOut, HpRadioReceiverInput.RfInOut, HpRadioCommissioning.Confirmed))], host, host);
    }
    private static CatalogRequest Request(int index) => index switch { < 2 => new CounterMeasureFrequencyRequest(), 2 => new DataAcquisitionMeasureVoltageDcRequest(7, null), _ => new RfReceiverMeasureFrequencyRequest() };
    internal static async Task Run()
    {
        for (var index = 0; index < 6; index++)
        {
            var candidate = new DeviceCandidate(new("hpib", "fixture"), new());
            foreach (var stallWrite in new[] { false, true })
            {
            if (stallWrite && index < 3) continue;
            foreach (var ownership in new[] { ConnectionOwnership.Owned, ConnectionOwnership.Borrowed })
            {
                var host = new Host { StallWrite = stallWrite }; var factory = Factory(index, host); var selection = new DriverSelection(DriverId: factory.Descriptor.Id);
                var handle = await new DriverRegistry([factory]).OpenNativeAsync(candidate, selection, ownership: ownership);
                using (var before = new CancellationTokenSource())
                {
                    before.Cancel(); try { await handle.InvokeAsync(Request(index), before.Token); throw new InvalidOperationException("Expected pre-cancellation"); } catch (OperationCanceledException) { }
                    Require(host.Calls == 0 && !handle.RecoveryRequired, "Pre-IO cancellation leaves session usable");
                }
                using (var during = new CancellationTokenSource())
                {
                    var task = handle.InvokeAsync(Request(index), during.Token); await host.Started.Task; during.Cancel();
                    try { await task; throw new InvalidOperationException("Expected cancellation"); } catch (OperationCanceledException) { }
                    Require(host.Active == 0 && handle.RecoveryRequired, "Native cancellation ends accesses and quarantines");
                }
                var calls = host.Calls;
                try { await handle.InvokeAsync(Request(index)); throw new InvalidOperationException("Expected quarantine"); } catch (DriverException e) { Require(e.Kind == DriverErrorKind.SessionNeedsRecovery, "Quarantine kind"); }
                await handle.CloseAsync(); Require(host.Calls == calls && host.Closes == (ownership == ConnectionOwnership.Owned ? 1 : 0), "No detached access; ownership close");
            }
            foreach (var native in new[] { false, true })
            {
                var host = new Host { Deadline = true, StallWrite = stallWrite }; var factory = Factory(index, host); var registry = new DriverRegistry([factory]); var selection = new DriverSelection(DriverId: factory.Descriptor.Id);
                var started = Stopwatch.StartNew();
                if (native)
                {
                    var handle = await registry.OpenNativeAsync(candidate, selection);
                    try { await handle.InvokeAsync(Request(index)); throw new InvalidOperationException("Expected deadline"); } catch (DriverException e) { Require(e.Kind == DriverErrorKind.Timeout, "Deadline kind"); }
                    try { await handle.InvokeAsync(Request(index)); throw new InvalidOperationException("Expected quarantine"); } catch (DriverException e) { Require(e.Kind == DriverErrorKind.SessionNeedsRecovery, "Quarantine kind"); }
                    await handle.CloseAsync();
                }
                else
                {
                    var handle = registry.Open(candidate, selection);
                    try { handle.Invoke(Request(index)); throw new InvalidOperationException("Expected deadline"); } catch (DriverException e) { Require(e.Kind == DriverErrorKind.Timeout, "Deadline kind"); }
                    try { handle.Invoke(Request(index)); throw new InvalidOperationException("Expected quarantine"); } catch (DriverException e) { Require(e.Kind == DriverErrorKind.SessionNeedsRecovery, "Quarantine kind"); }
                    handle.Close();
                }
                Require(started.Elapsed >= TimeSpan.FromMilliseconds(15) && started.Elapsed < TimeSpan.FromSeconds(5), "Independent deadline expires pending IO (timer-resolution tolerance)");
                Require(host.Calls == (stallWrite || index < 3 ? 1 : 3) && host.Closes == 1, "No retry after stalled IO; one close");
                Require(host.Active == 0, "Deadline completion ends native host access");
            }
            }
        }
        Console.WriteLine("PASS all six HP models native pre/during cancellation, owned/borrowed lifetime and independent pending-read deadlines; all RF models pending-write cancellation/deadlines (sync/native); no detached access or retries");
    }
    private sealed class Host : IScpiConnector, IMessageIo, IAsyncScpiConnector, IAsyncMessageIo
    {
        public bool Deadline, StallWrite; public int Calls, Active, Closes;
        public TaskCompletionSource Started { get; } = new(TaskCreationOptions.RunContinuationsAsynchronously);
        public IMessageIo Open(DriverOpenContext context) => this;
        public ValueTask<IAsyncMessageIo> OpenAsync(DriverOpenContext context, CancellationToken cancellationToken = default) => ValueTask.FromResult<IAsyncMessageIo>(this);
        public void Write(string message) { if (StallWrite) { Query(message); return; } Interlocked.Increment(ref Calls); }
        public string Query(string message)
        {
            Interlocked.Increment(ref Calls); using var pending = new ManualResetEventSlim(false);
            if (!pending.Wait(TimeSpan.FromMilliseconds(20))) throw new DriverException(DriverErrorKind.Timeout, "Independent deadline expired while response remained pending");
            throw new InvalidOperationException("Pending response unexpectedly completed");
        }
        public async ValueTask WriteAsync(string message, CancellationToken cancellationToken = default) { if (StallWrite) { await QueryAsync(message, cancellationToken); return; } cancellationToken.ThrowIfCancellationRequested(); Interlocked.Increment(ref Calls); }
        public async ValueTask<string> QueryAsync(string message, CancellationToken cancellationToken = default)
        {
            Interlocked.Increment(ref Calls); Interlocked.Increment(ref Active); Started.TrySetResult();
            using var timeout = CancellationTokenSource.CreateLinkedTokenSource(cancellationToken);
            if (Deadline) timeout.CancelAfter(TimeSpan.FromMilliseconds(20));
            try { await Task.Delay(Timeout.InfiniteTimeSpan, timeout.Token).ConfigureAwait(false); throw new InvalidOperationException("Pending response unexpectedly completed"); }
            catch (OperationCanceledException) when (Deadline && !cancellationToken.IsCancellationRequested) { throw new DriverException(DriverErrorKind.Timeout, "Independent deadline expired while response remained pending"); }
            finally { Interlocked.Decrement(ref Active); }
        }
        public void Dispose() => Interlocked.Increment(ref Closes);
    }
}
