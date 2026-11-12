using InstrumentComponents.Drivers;
using InstrumentComponents.Drivers.Catalog;
using InstrumentComponents.Drivers.Keysight;
using InstrumentComponents.Runtime;
using InstrumentComponents.Scpi;
using InstrumentComponents.TestKit;
using Identity = InstrumentComponents.Drivers.DeviceIdentity;

internal static class HpCounterProof
{
    private static void Require(bool condition, string message) { if (!condition) throw new InvalidOperationException(message); }
    private static DeviceCandidate Candidate(Hp5334Model model) => new(new("hpib", "GPIB0::3::INSTR"),
        new Identity("HP", model == Hp5334Model.Hp5334A ? "5334A" : "5334B"));
    private static Hp5334Factory Factory(Hp5334Model model, Script script, IAsyncScpiConnector? native = null) =>
        new(model, Hp5334Commissioning.NativeInternalGatingConfirmed, script, native);
    private static void Error(Action action, DriverErrorKind kind)
    {
        try { action(); } catch (DriverException error) { Require(error.Kind == kind, $"Expected {kind}, got {error.Kind}"); return; }
        throw new InvalidOperationException("Expected " + kind);
    }
    private static async Task ErrorAsync(Func<Task> action, DriverErrorKind kind)
    {
        try { await action(); } catch (DriverException error) { Require(error.Kind == kind, $"Expected {kind}, got {error.Kind}"); return; }
        throw new InvalidOperationException("Expected " + kind);
    }

    public static async Task Run()
    {
        foreach (var model in new[] { Hp5334Model.Hp5334A, Hp5334Model.Hp5334B })
        {
            var empty = new Script([]);
            foreach (var mode in new[] { Hp5334Commissioning.Unknown, Hp5334Commissioning.Ciil, (Hp5334Commissioning)99 })
                Error(() => new Hp5334Factory(model, mode, empty), DriverErrorKind.InvalidArgument);
            Error(() => new Hp5334Factory((Hp5334Model)99, Hp5334Commissioning.NativeInternalGatingConfirmed, empty), DriverErrorKind.InvalidArgument);
            var factory = Factory(model, empty);
            var registry = new DriverRegistry([factory]);
            foreach (var manufacturer in new[] { "HP", "Hewlett-Packard", " hp ", "HEWLETT-PACKARD" })
                Require(registry.Select(Candidate(model) with { Identity = Candidate(model).Identity with { Manufacturer = manufacturer } }).DriverId == factory.DriverId, "Passive identity selection");
            Error(() => registry.Select(Candidate(model) with { Endpoint = new("scpi", "fixture") }), DriverErrorKind.DriverNotFound);
            Error(() => registry.Open(Candidate(model), required: new HashSet<Operation> { Operation.CatalogQueryIdentity }), DriverErrorKind.UnsupportedOperation);
            Require(empty.Opens == 0 && !factory.ExecutionModes.Contains(ExecutionMode.NativeAsync), "No acquisition/probing for rejected requirements or absent native connector");
            var borrowed = registry.Open(Candidate(model), ownership: ConnectionOwnership.Borrowed);
            borrowed.Close(); borrowed.Close(); borrowed.Dispose();
            Require(empty.Opens == 1 && empty.Closes == 0, "Borrowed resource preserved"); empty.AssertComplete();

            var script = new Script([new("close")]);
            var handle = new DriverRegistry([Factory(model, script)]).Open(Candidate(model));
            foreach (var seconds in new[] { double.NaN, double.PositiveInfinity, double.NegativeInfinity })
                Error(() => handle.Invoke(new CounterSetGateTimeRequest(seconds)), DriverErrorKind.InvalidArgument);
            handle.Invoke(new CounterSelectChannelRequest(1));
            handle.Close(); handle.Close(); handle.Dispose(); script.AssertComplete(); Require(script.Closes == 1, "Owned close once");

            foreach (var failure in new[] { DriverErrorKind.Timeout, DriverErrorKind.Transport, DriverErrorKind.Parse })
            {
                script = new Script([new("query", "FN1,HS0,MD1,WA1,RE", failure == DriverErrorKind.Parse ? "F" : "", failure == DriverErrorKind.Parse ? null : failure), new("close")]);
                handle = new DriverRegistry([Factory(model, script)]).Open(Candidate(model));
                Error(() => handle.Invoke(new CounterMeasureFrequencyRequest()), failure);
                Error(() => handle.Invoke(new CounterMeasureFrequencyRequest()), DriverErrorKind.SessionNeedsRecovery);
                Error(handle.Recover, DriverErrorKind.SessionNeedsRecovery);
                Require(script.Events.Count == 1, "No recovery/reset/retry traffic");
                handle.Close(); script.AssertComplete();
            }

            script = new Script([new("close", Failure: DriverErrorKind.Transport)]);
            handle = new DriverRegistry([Factory(model, script)]).Open(Candidate(model));
            Error(() => handle.Close(), DriverErrorKind.Shutdown); Error(() => handle.Close(), DriverErrorKind.Shutdown);
            // Close outcome remains failed; disposal must not retry the physical close.
            try { handle.Dispose(); } catch (DriverException) { }
            Require(script.Closes == 1, "Failed close attempted once"); script.AssertComplete();

            var pending = new PendingIo();
            var native = await new DriverRegistry([Factory(model, new Script([]), pending)]).OpenNativeAsync(Candidate(model));
            using (var before = new CancellationTokenSource())
            {
                before.Cancel();
                try { await native.InvokeAsync(new CounterMeasureFrequencyRequest(), before.Token); throw new InvalidOperationException("Expected cancellation"); }
                catch (OperationCanceledException) { }
                Require(pending.Queries == 0 && !native.RecoveryRequired, "Pre-IO cancellation");
            }
            using (var during = new CancellationTokenSource())
            {
                var task = native.InvokeAsync(new CounterMeasureFrequencyRequest(), during.Token);
                await pending.Started.Task;
                during.Cancel();
                try { await task; throw new InvalidOperationException("Expected cancellation"); } catch (OperationCanceledException) { }
                Require(pending.Active == 0 && pending.Queries == 1 && native.RecoveryRequired, "Cancellation ends host access and quarantines");
            }
            await ErrorAsync(async () => { await native.InvokeAsync(new CounterMeasureFrequencyRequest()); }, DriverErrorKind.SessionNeedsRecovery);
            await ErrorAsync(() => native.RecoverAsync(), DriverErrorKind.SessionNeedsRecovery);
            await native.CloseAsync(); Require(pending.Closes == 1 && pending.Queries == 1, "Native close/no detached traffic");
        }
        Console.WriteLine("PASS HP5334A/B passive selection, commissioning, nonfinite arguments, owned/borrowed lifetime, quarantine and native cancellation");
    }

    private sealed class PendingIo : IAsyncScpiConnector, IAsyncMessageIo
    {
        public int Queries, Closes, Active;
        public TaskCompletionSource Started { get; } = new(TaskCreationOptions.RunContinuationsAsynchronously);
        public ValueTask<IAsyncMessageIo> OpenAsync(DriverOpenContext context, CancellationToken cancellationToken = default) => ValueTask.FromResult<IAsyncMessageIo>(this);
        public ValueTask WriteAsync(string message, CancellationToken cancellationToken = default) => throw new InvalidOperationException("Unexpected write");
        public async ValueTask<string> QueryAsync(string message, CancellationToken cancellationToken = default)
        {
            Require(message == "FN1,HS0,MD1,WA1,RE", "Native exact command");
            Interlocked.Increment(ref Queries); Interlocked.Increment(ref Active); Started.TrySetResult();
            try { await Task.Delay(Timeout.InfiniteTimeSpan, cancellationToken).ConfigureAwait(false); return ""; }
            finally { Interlocked.Decrement(ref Active); }
        }
        public void Dispose() => Interlocked.Increment(ref Closes);
    }
}
