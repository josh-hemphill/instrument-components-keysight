"""Independent TDS1002 primary-manual/spec oracle. Reads no production files.

Fixture: TDS2CMA GPIB, channel1 voltage probe10X, channel2 probe1X,
qualified normal-trigger/main/YT mode, DESE255 and clean status.
Bodies5.16E0 and28.75E6 originate in own-applicable manual examples;
all control scripts and negative/error cases are synthetic software evidence.
"""
import json
from pathlib import Path
root=Path(__file__).parent

def call(op,**args):return {'operation':op,'arguments':args}
def q(message,response):return {'kind':'query','message':message,'response':response}
def w(message):return {'kind':'write','message':message}
def prefix():return [w(':HEAD OFF'),w(':VERB ON'),q(':DESE?','255'),q('*ESR?','0')]
def end():return [q('*ESR?','0')]
def fmt(x):
 s=f'{x:.11E}';a,e=s.split('E');n=int(e);return a+'E'+('+'if n>=0 else'-')+f'{abs(n):02d}'
def c(name,calls,script,result=None,error=None):
 r={'id':'tektronix.tds1002|E|'+name,'calls':calls,'script':script+[{'kind':'close'}]};r['expectedError'if error else'expected']=error or result;return r
unit={'kind':'unit'};rows=[]

def qty(value,unit):return {'kind':'quantity','value':value,'unit':unit,'status':'Valid'}
def measure(channel,kind='Vpp',body=None):
 probe=10 if channel==1 else 1
 waveform='PK2PK'if kind=='Vpp'else'FREQ';type_reply='PK2PK'if kind=='Vpp'else'FREQUENCY';units='"V"'if kind=='Vpp'else'"Hz"'
 if body is None:body='5.16E0'if kind=='Vpp'else'28.75E6'
 return prefix()+[
  q(':HOR:VIEW?','MAIN'),q(':DISP:FORM?','YT'),q(':TRIG:MAIN:MODE?','NORMAL'),q(f':SEL:CH{channel}?','1'),q(f':CH{channel}:PRO?',str(probe)),
  w(':ACQ:STATE STOP'),w(':ACQ:MODE SAMPLE'),q(':ACQ:MODE?','SAMPLE'),w(':ACQ:STOPA SEQ'),q(':ACQ:STOPA?','SEQUENCE'),w(':ACQ:STATE RUN'),q('*OPC?','1'),q(':ACQ:STATE?','0'),q(':ACQ:NUMACQ?','1'),
  w(f':MEASU:IMM:SOU CH{channel}'),w(':MEASU:IMM:TYP '+waveform),q(':MEASU:IMM:SOU?',f'CH{channel}'),q(':MEASU:IMM:TYP?',type_reply),q(':MEASU:IMM:UNI?',units),q(':MEASU:IMM:VAL?',body),q('*ESR?','0')]

timebase=[float('5e-9')]+[float(str(m)+'e'+str(e))for e in range(-8,2)for m in[1,2.5,5]]
assert len(timebase)==31
for i,t in enumerate(timebase):rows.append(c('timebase-nominal-'+str(i),[call('OscilloscopeSetTimebaseScale',seconds_per_div=t)],prefix()+[q(':HOR:VIEW?','MAIN'),w(':HOR:MAIN:SCA '+fmt(t))]+end(),unit))
for name,body,v in [('minimum','5E-9',5e-9),('ordinary','2.5E-6',2.5e-6),('maximum','5.0E1',50)]:rows.append(c('timebase-read-'+name,[call('OscilloscopeReadTimebaseScale')],prefix()+[q(':HOR:VIEW?','MAIN'),q(':HOR:MAIN:SCA?',body)]+end(),qty(v,'Second')))
for channel,probe in[(1,10),(2,1)]:
 for m in[2,5,10,20,50,100,200,500,1000,2000,5000]:
  v=(m*probe)/1000.0
  rows.append(c(f'channel-{channel}-scale-{m}mv',[call('OscilloscopeSetChannelScale',channel=channel,volts_per_div=v)],prefix()+[q(f':CH{channel}:PRO?',str(probe)),w(f':CH{channel}:SCA '+fmt(v))]+end(),unit))
 for enabled in[False,True]:rows.append(c(f'channel-{channel}-display-{enabled}',[call('OscilloscopeSetChannelDisplay',channel=channel,enabled=enabled)],prefix()+[w(f':SEL:CH{channel} '+('ON'if enabled else'OFF'))]+end(),unit))
 for coupling in['AC','DC','GND']:rows.append(c(f'channel-{channel}-coupling-'+coupling,[call('OscilloscopeSetChannelCoupling',channel=channel,coupling=coupling)],prefix()+[w(f':CH{channel}:COUP '+coupling)]+end(),unit))
 for kind,val,unitname in[('Vpp',5.16,'Volt'),('Frequency',28.75e6,'Hertz')]:rows.append(c(f'channel-{channel}-fresh-'+kind,[call('OscilloscopeMeasure'+kind,channel=channel)],measure(channel,kind),qty(val,unitname)))
for name,steps in[('Run',[w(':ACQ:STOPA RUNSTOP'),w(':ACQ:STATE RUN')]),('Stop',[w(':ACQ:STATE STOP')]),('Single',[w(':ACQ:STOPA SEQ'),w(':ACQ:STATE RUN')])]:rows.append(c('acquisition-'+name,[call('Oscilloscope'+name)],prefix()+steps+end(),unit))
rows.append(c('repeat-measurements-are-fresh',[call('OscilloscopeMeasureVpp',channel=1),call('OscilloscopeMeasureFrequency',channel=2)],measure(1)+measure(2,'Frequency'),qty(28.75e6,'Hertz')))
rows.append(c('single-is-start-only-then-fresh-restart',[call('OscilloscopeSingle'),call('OscilloscopeMeasureVpp',channel=1)],prefix()+[w(':ACQ:STOPA SEQ'),w(':ACQ:STATE RUN')]+end()+measure(1),qty(5.16,'Volt')))
for v in[0,-1,4.999e-9,50.0001,1e-6+2.117582368135751e-22]:rows.append(c('invalid-timebase-'+str(v),[call('OscilloscopeSetTimebaseScale',seconds_per_div=v)],[],error='InvalidArgument'))
for channel in[0,3,4294967295]:
 for op,args in[('OscilloscopeSetChannelScale',{'volts_per_div':1}),('OscilloscopeSetChannelDisplay',{'enabled':True}),('OscilloscopeSetChannelCoupling',{'coupling':'DC'}),('OscilloscopeMeasureVpp',{}),('OscilloscopeMeasureFrequency',{})]:rows.append(c(op+'-invalid-channel-'+str(channel),[call(op,channel=channel,**args)],[],error='InvalidArgument'))
for channel,values in[(1,[0,-1,.01999,50.001,.03]),(2,[0,-1,.00199,5.001,.003])]:
 for i,v in enumerate(values):rows.append(c(f'invalid-channel-{channel}-scale-{i}',[call('OscilloscopeSetChannelScale',channel=channel,volts_per_div=v)],[],error='InvalidArgument'))
for value in['ac','Ac','GROUND','DCX','1']:
 rows.append(c('invalid-coupling-'+value,[call('OscilloscopeSetChannelCoupling',channel=1,coupling=value)],[],error='InvalidArgument'))
for i,(op,args)in enumerate([('OscilloscopeCaptureVoltageTrace',{'channel':1}),('OscilloscopeSetTriggerSource',{'source':'CH1'}),('OscilloscopeSetTriggerLevel',{'volts':1}),('OscilloscopeSetTriggerSlope',{'slope':'RISE'})]):rows.append(c('deferred-operation-'+str(i),[call(op,**args)],[],error='UnsupportedOperation'))
for i,(body,error)in enumerate([('0','Device'),('60','Device'),('256','Parse'),('-1','Parse'),('255.0','Parse'),(' 255','Parse'),('255 ','Parse'),('NaN','Parse')]):
 steps=prefix()[:3];steps[-1]['response']=body
 rows.append(c('dese-reject-'+str(i),[call('OscilloscopeStop')],steps,error=error))
for n,body in[('command-error','32'),('execution-warning','16'),('power-on','128')]:
 steps=prefix();steps[-1]['response']=body
 rows.append(c('pre-esr-'+n,[call('OscilloscopeStop')],steps,error='Device'))
for i,body in enumerate(['-1','256','1.0','0,0',' 0','NaN']):
 steps=prefix();steps[-1]['response']=body
 rows.append(c('pre-esr-malformed-'+str(i),[call('OscilloscopeStop')],steps,error='Parse'))
# Signed integer NR1 is legal for registers/probes; Boolean/OPC remain exact.
steps=prefix();steps[2]['response']='+0255';steps[3]['response']='+000'
rows.append(c('signed-register-nr1',[call('OscilloscopeStop')],steps+[w(':ACQ:STATE STOP'),q('*ESR?','+0')],unit))
for n,body in[('warning','16'),('ignored-set-trigger-view','16'),('command-error','32')]:rows.append(c('post-esr-'+n,[call('OscilloscopeMeasureVpp',channel=1)],measure(1)[:-1]+[q('*ESR?',body)],error='Device'))
# Indexes are explicit positions in the normative script, not derived from backend plans.
checks={4:[('WINDOW','Device'),('ZONE','Device'),('MAINX','Parse')],5:[('XY','Device'),('YTX','Parse')],6:[('AUTO','Device'),('NORM','Parse')],7:[('0','Device'),('+1','Parse')],8:[('1','Device'),('20','Parse')],11:[('AVERAGE','Device'),('PEAKDETECT','Device'),('SAM','Parse')],13:[('RUNSTOP','Device'),('SEQ','Parse')],15:[('0','Parse'),('1 ','Parse')],16:[('1','Device'),('OFF','Parse')],17:[('0','Device'),('2','Device'),('-1','Parse'),('2147483648','Parse')],20:[('CH2','Device'),('MATH','Parse')],21:[('MEAN','Device'),('CRMS','Device'),('PK2','Parse')],22:[('"Hz"','Device'),('"s"','Device'),('V','Parse'),('"A"','Parse')]}
for i,alts in checks.items():
 for j,(body,error)in enumerate(alts):
  steps=measure(1)[:i+1];assert steps[-1]['kind']=='query',(i,steps[-1]);steps[-1]['response']=body
  rows.append(c(f'measure-guard-{i}-{j}',[call('OscilloscopeMeasureVpp',channel=1)],steps,error=error))
for kind in['Vpp','Frequency']:
 for name,body,v in[('ordinary','5.16E0',5.16),('lower-e','1.25e+0',1.25),('signed','+1.25E+0',1.25),('leading-zeros','0001.25E0',1.25)]:rows.append(c(kind+'-scalar-'+name,[call('OscilloscopeMeasure'+kind,channel=1)],measure(1,kind,body),qty(v,'Volt'if kind=='Vpp'else'Hertz')))
 rows.append(c(kind+'-zero',[call('OscilloscopeMeasure'+kind,channel=1)],measure(1,kind,'0E0')if kind=='Vpp'else measure(1,kind,'0E0')[:-1],qty(0,'Volt')if kind=='Vpp'else None,error=None if kind=='Vpp'else'Device'))
 for name,body,error in[('invalid-sentinel','9.9E37','Device'),('negative','-1.25E0','Device'),('negative-sentinel','-9.9E37','Parse'),('giant','1E30','Parse'),('plain-decimal','1.25','Parse'),('leading-space',' 1E0','Parse'),('trailing-space','1E0 ','Parse'),('missing-leading','.5E0','Parse'),('missing-fraction','1.E0','Parse'),('csv','1E0,2E0','Parse'),('header',':MEASUREMENT:IMMED:VALUE 1E0','Parse'),('binary','#131E0','Parse'),('nan','NaN','Parse'),('infinity','Infinity','Parse'),('overflow','1E999','Parse'),('underflow','1E-999','Parse')]:rows.append(c(kind+'-scalar-reject-'+name,[call('OscilloscopeMeasure'+kind,channel=1)],measure(1,kind,body)[:-1],error=error))
for i,body,error in[(4,'WINDOW','Device'),(5,'0E0','Device'),(5,'2.6E-6','Device'),(5,'9.9E37','Parse'),(5,'2.5e-6 ','Parse'),(5,'2.5e-6s','Parse')]:
 steps=prefix()+[q(':HOR:VIEW?','MAIN'),q(':HOR:MAIN:SCA?','2.5E-6')];steps=steps[:i+1];steps[-1]['response']=body
 rows.append(c('timebase-read-reject-'+str(i)+'-'+str(len(rows)),[call('OscilloscopeReadTimebaseScale')],steps,error=error))
# Failure at every message of the full fresh scalar transaction and all control actions.
for kind in['Vpp','Frequency']:
 for i,step in enumerate(measure(1,kind)):
  for error in['Transport','Timeout']:
   steps=measure(1,kind)[:i]+[{k:v for k,v in step.items()if k!='response'}|{'error':error}]
   rows.append(c(f'{kind}-failure-step-{i}-{error}',[call('OscilloscopeMeasure'+kind,channel=1)],steps,error=error))
for op,args,steps in[('OscilloscopeSetTimebaseScale',{'seconds_per_div':1e-3},prefix()+[q(':HOR:VIEW?','MAIN'),w(':HOR:MAIN:SCA 1.00000000000E-03')]+end()),('OscilloscopeSetChannelScale',{'channel':1,'volts_per_div':1},prefix()+[q(':CH1:PRO?','10'),w(':CH1:SCA 1.00000000000E+00')]+end()),('OscilloscopeSetChannelDisplay',{'channel':1,'enabled':False},prefix()+[w(':SEL:CH1 OFF')]+end()),('OscilloscopeSetChannelCoupling',{'channel':1,'coupling':'AC'},prefix()+[w(':CH1:COUP AC')]+end()),('OscilloscopeRun',{},prefix()+[w(':ACQ:STOPA RUNSTOP'),w(':ACQ:STATE RUN')]+end()),('OscilloscopeStop',{},prefix()+[w(':ACQ:STATE STOP')]+end()),('OscilloscopeSingle',{},prefix()+[w(':ACQ:STOPA SEQ'),w(':ACQ:STATE RUN')]+end())]:
 for i in range(4,len(steps)):
  step=steps[i];failed={k:v for k,v in step.items()if k!='response'}|{'error':'Transport'}
  rows.append(c(op+f'-action-failure-{i}',[call(op,**args)],steps[:i]+[failed],error='Transport'))
assert len({x['id']for x in rows})==len(rows)
(root/'tds1002.catalog-cases.json').write_text(json.dumps({'schemaVersion':1,'contractMajor':1,'description':'Independent own-model TDS1002 manual/spec oracle; control scripts and parser/failure cases are synthetic software evidence, not bench observations.','cases':rows},indent=2)+'\n')
(root/'tds1002-fixture.json').write_text(json.dumps({'driverId':'tektronix.tds1002','endpointScheme':'scpi','endpointAddress':'fixture','communicationModule':'Tds2Cma','channel1Probe':'X10','channel2Probe':'X1','commissioning':'QualifiedGpibNormalModeConfirmed','readbackDefaults':{'DESE':255,'ESR':0,'mainView':'MAIN','displayFormat':'YT','triggerMode':'NORMAL'}},indent=2)+'\n')
print('PASS TDS1002',len(rows),'independent cases')
