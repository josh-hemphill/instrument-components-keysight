using System.Diagnostics;
using InstrumentComponents.Drivers;
using InstrumentComponents.Drivers.Catalog;
using InstrumentComponents.Runtime;
using InstrumentComponents.Scpi;
using InstrumentComponents.TestKit;

internal static class QualifiedPendingProof
{
    private static readonly(string Read,string? Write)[] Scopes=[
        ("keysight.34401a|E|VoltageDc-auto",null),
        ("keysight.33120a|E|read-frequency","keysight.33120a|E|frequency-1000"),
        ("keysight.53132a|E|Frequency-channel-1","keysight.53132a|E|Frequency-channel-1"),
        ("keysight.hp3585b|E|marker-X-ordinary","keysight.hp3585b|E|marker-X-ordinary"),
        ("keysight.34980a|E|34937a-state-7-False","keysight.34980a|E|34937a-SwitchCloseChannel-7"),
        ("keysight.34980a|E|34922a-state-101-False","keysight.34980a|E|34922a-SwitchCloseChannel-101"),
        ("keysight.34980a|E|34950a-read-7-0","keysight.34980a|E|34950a-write-13-0"),
        ("keysight.34980a|E|34950a-read-29-0","keysight.34980a|E|34950a-write-61-0")];
    private static DeviceCandidate Candidate(string id)=>new(new(id.StartsWith("keysight.hp",StringComparison.Ordinal)?"hpib":"scpi","fixture"),new());
    private static DriverSelection Selection(string id)=>new(DriverId:id.Split('|')[0]);
    private static void Require(bool value,string message){if(!value)throw new InvalidOperationException(message);}
    private static async Task Error(DriverErrorKind kind,Func<Task<CatalogReply>> action)
    {try{await action();throw new InvalidOperationException("Missing expected "+kind);}catch(DriverException error){Require(error.Kind==kind,"Expected "+kind+", actual "+error.Kind);}}
    internal static async Task Run()
    {
        var cases=GeneratedDrivers.GeneratedCases.Create().ToDictionary(c=>c.Id,StringComparer.Ordinal);
        foreach(var(readId,writeId)in Scopes)
        {
            foreach(var(write,id)in new[]{(false,readId),(true,writeId)})
            {
                if(id is null)continue;var test=cases[id];var request=test.Calls.Single();
                foreach(var ownership in new[]{ConnectionOwnership.Owned,ConnectionOwnership.Borrowed})
                {
                    var owned=ownership==ConnectionOwnership.Owned;var host=new Host(test,write,owned,false,true);
                    var factory=QualifiedFixture.Factory(id,host,host)!;var handle=await new DriverRegistry([factory]).OpenNativeAsync(Candidate(id),Selection(id),ownership:ownership);
                    using(var before=new CancellationTokenSource())
                    {
                        before.Cancel();try{await handle.InvokeAsync(request,before.Token);throw new InvalidOperationException("Expected pre-cancellation");}catch(OperationCanceledException){}
                        Require(host.Calls==0&&!handle.RecoveryRequired,"Pre-IO cancellation quarantined or accessed host");
                    }
                    using(var during=new CancellationTokenSource())
                    {
                        var task=handle.InvokeAsync(request,during.Token);await host.Started.Task.WaitAsync(TimeSpan.FromSeconds(5));Require(host.Active==1,"Target is not actually pending");during.Cancel();
                        try{await task;throw new InvalidOperationException("Expected active cancellation");}catch(OperationCanceledException){}
                    }
                    Require(host.Active==0&&handle.RecoveryRequired,"Cancellation returned before host access ended");var calls=host.Calls;
                    await Error(DriverErrorKind.SessionNeedsRecovery,()=>handle.InvokeAsync(request));await handle.CloseAsync();await Task.Yield();
                    Require(host.Calls==calls&&host.Script.Closes==(owned?1:0),"Detached IO or ownership mismatch");host.Script.AssertComplete();
                }
                foreach(var native in new[]{false,true})
                {
                    var host=new Host(test,write,true,true,native);var factory=QualifiedFixture.Factory(id,host,host)!;var registry=new DriverRegistry([factory]);var watch=Stopwatch.StartNew();
                    if(native)
                    {
                        var handle=await registry.OpenNativeAsync(Candidate(id),Selection(id));
                        await Error(DriverErrorKind.Timeout,()=>handle.InvokeAsync(request));await Error(DriverErrorKind.SessionNeedsRecovery,()=>handle.InvokeAsync(request));await handle.CloseAsync();
                    }
                    else
                    {
                        var handle=registry.Open(Candidate(id),Selection(id));
                        foreach(var expected in new[]{DriverErrorKind.Timeout,DriverErrorKind.SessionNeedsRecovery})
                        {try{handle.Invoke(request);throw new InvalidOperationException("Expected "+expected);}catch(DriverException error){Require(error.Kind==expected,"Sync deadline/quarantine kind");}}
                        handle.Close();
                    }
                    Require(watch.Elapsed>=TimeSpan.FromMilliseconds(15)&&watch.Elapsed<TimeSpan.FromSeconds(5),"Independent pending deadline duration");
                    Require(host.Calls==host.PendingAt+1&&host.Active==0&&host.Script.Closes==1,"Deadline did not end host IO or retried");host.Script.AssertComplete();
                }
            }
            var first=cases[readId];var second=readId.Contains("34950a",StringComparison.Ordinal)?cases["keysight.34980a|E|34937a-SwitchCloseChannel-7"]:first;
            var shared=new Host(first,false,true,false,true,second);var f=QualifiedFixture.Factory(readId,shared,shared)!;
            var h=await new DriverRegistry([f]).OpenNativeAsync(Candidate(readId),Selection(readId));
            var one=h.InvokeAsync(first.Calls.Single());await shared.Started.Task.WaitAsync(TimeSpan.FromSeconds(5));var callsAtPause=shared.Calls;
            var two=h.InvokeAsync(second.Calls.Single());await Task.Yield();Require(!two.IsCompleted&&shared.Calls==callsAtPause,"Second view interleaved with a whole transaction");
            shared.Release.TrySetResult(shared.Response);Require(Equals(await one,first.Expected)&&Equals(await two,second.Expected),"Concurrent operation result");
            await h.CloseAsync();shared.Script.AssertComplete();Require(shared.Active==0&&shared.Script.Closes==1,"Shared parent cleanup");
        }
        Console.WriteLine("PASS all eight Agilent items: actual pending native reads/writes, independent sync/native deadlines, cancellation cleanup, owned/borrowed lifetime and whole-operation/sibling-view serialization");
    }
    private sealed class Host:IScpiConnector,IAsyncScpiConnector
    {
        public readonly Script Script;public readonly int PendingAt;public readonly string Message,Response;private readonly bool _write,_deadline,_nativeOnly;
        public int Calls,Active;public TaskCompletionSource Started{get;}=new(TaskCreationOptions.RunContinuationsAsynchronously);
        public TaskCompletionSource<string> Release{get;}=new(TaskCreationOptions.RunContinuationsAsynchronously);
        public Host(CatalogCase test,bool write,bool owned,bool deadline,bool nativeOnly,CatalogCase? second=null)
        {
            Require(test.Calls.Count==1,"Single-operation pending fixture");_write=write;_deadline=deadline;_nativeOnly=nativeOnly;
            PendingAt=Enumerable.Range(0,test.Script.Count).Last(i=>test.Script[i].Kind==(write?"write":"query"));Message=test.Script[PendingAt].Message;Response=test.Script[PendingAt].Response;
            var prefix=test.Script.Take(PendingAt).ToList();
            if(second is not null){Require(PendingAt+2==test.Script.Count,"Pending query is final IO");prefix.AddRange(second.Script.Where(s=>s.Kind!="close"));}
            if(owned)prefix.Add(new("close"));Script=new(prefix);
        }
        public bool Step(string message,bool write)
        {
            var index=Interlocked.Increment(ref Calls)-1;if(index!=PendingAt)return false;
            Require(message==Message&&write==_write,"Pending target traffic");return true;
        }
        public string PendingSync()
        {
            Interlocked.Increment(ref Active);Started.TrySetResult();
            try{using var pending=new ManualResetEventSlim(false);if(!pending.Wait(TimeSpan.FromMilliseconds(20)))throw new DriverException(DriverErrorKind.Timeout,"Independent pending-host deadline");throw new InvalidOperationException("Unexpected response progress");}
            finally{Interlocked.Decrement(ref Active);}
        }
        public async ValueTask<string> Pending(CancellationToken cancellationToken)
        {
            Interlocked.Increment(ref Active);Started.TrySetResult();using var timeout=CancellationTokenSource.CreateLinkedTokenSource(cancellationToken);
            if(_deadline)timeout.CancelAfter(TimeSpan.FromMilliseconds(20));
            try{if(_deadline){await Task.Delay(Timeout.InfiniteTimeSpan,timeout.Token).ConfigureAwait(false);throw new InvalidOperationException("Unexpected response progress");}return await Release.Task.WaitAsync(timeout.Token).ConfigureAwait(false);}
            catch(OperationCanceledException)when(_deadline&&!cancellationToken.IsCancellationRequested){throw new DriverException(DriverErrorKind.Timeout,"Independent pending-host deadline");}
            finally{Interlocked.Decrement(ref Active);}
        }
        public IMessageIo Open(DriverOpenContext context){Require(!_nativeOnly,"Native path used sync acquisition");return new SyncIo(this,Script.Open(context));}
        public async ValueTask<IAsyncMessageIo> OpenAsync(DriverOpenContext context,CancellationToken cancellationToken=default)=>new NativeIo(this,await Script.OpenAsync(context,cancellationToken));
        private sealed class SyncIo(Host host,IMessageIo inner):IMessageIo
        {
            public void Write(string message){Require(!host._nativeOnly,"Native path used sync write");if(host.Step(message,true))host.PendingSync();else inner.Write(message);}
            public string Query(string message){Require(!host._nativeOnly,"Native path used sync query");return host.Step(message,false)?host.PendingSync():inner.Query(message);}
            public void Dispose()=>inner.Dispose();
        }
        private sealed class NativeIo(Host host,IAsyncMessageIo inner):IAsyncMessageIo
        {
            public async ValueTask WriteAsync(string message,CancellationToken cancellationToken=default){if(host.Step(message,true))await host.Pending(cancellationToken);else await inner.WriteAsync(message,cancellationToken);}
            public async ValueTask<string> QueryAsync(string message,CancellationToken cancellationToken=default)=>host.Step(message,false)?await host.Pending(cancellationToken):await inner.QueryAsync(message,cancellationToken);
            public void Dispose()=>inner.Dispose();
        }
    }
}
