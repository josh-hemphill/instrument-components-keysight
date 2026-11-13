"""Manual/spec-authored golden data; reads no production files or implementation plans."""
import copy,json
from pathlib import Path
root=Path(__file__).parent

def call(op,**args):return {'operation':op,'arguments':args}
def q(msg,body):return {'kind':'query','message':msg,'response':body}
def w(msg):return {'kind':'write','message':msg}
def qty(value,unit,status='Valid'):return {'kind':'quantity','value':value,'unit':unit,'status':status}
def case(model,name,calls,script,expected=None,error=None):
 r={'id':f'keysight.{model}|E|{name}','calls':calls,'script':script+[{'kind':'close'}]}
 r['expectedError' if error else 'expected']=error or expected
 return r
def save(model,cases):
 (root/f'{model}.catalog-cases.json').write_text(json.dumps({'schemaVersion':1,'contractMajor':1,'description':'Independent manual/spec-derived traffic, not hardware observations; generator reads no production plans.','cases':cases},indent=2)+'\n')
 print(model,len(cases))

a=[]
functions=[('VoltageDc','VOLT:DC','Volt',1000,'1.00000000000E+03'),('VoltageAc','VOLT:AC','Volt',750,'7.50000000000E+02'),('CurrentDc','CURR:DC','Ampere',3,'3.00000000000E+00'),('CurrentAc','CURR:AC','Ampere',3,'3.00000000000E+00'),('Resistance','RES','Ohm',100000000,'1.00000000000E+08'),('Resistance2Wire','RES','Ohm',100000000,'1.00000000000E+08'),('Resistance4Wire','FRES','Ohm',100000000,'1.00000000000E+08')]
for name,wire,unit,limit,formatted in functions:
 op='DmmMeasure'+name
 a.append(case('34401a',name+'-auto',[call(op,range=None)],[q(':MEAS:'+wire+'? DEF,DEF','+1.25000000E+00')],qty(1.25,unit)))
 a.append(case('34401a',name+'-maximum-range',[call(op,range=limit)],[q(':MEAS:'+wire+'? '+formatted+',DEF','+1.25000000E+00')],qty(1.25,unit)))
 a.append(case('34401a',name+'-non-nominal-range',[call(op,range=0.025)],[q(':MEAS:'+wire+'? 2.50000000000E-02,DEF','+1.25000000E-03')],qty(.00125,unit)))
 a.append(case('34401a',name+'-overload',[call(op,range=None)],[q(':MEAS:'+wire+'? DEF,DEF','+9.90000000E+37')],qty(0,unit,'Overload')))
 for n,bad in [('zero',0),('negative',-1),('above-max',limit*1.000001)]:
  a.append(case('34401a',name+'-range-'+n,[call(op,range=bad)],[],error='InvalidArgument'))
for n,body,value in [('negative','-1.25000000E+00',-1.25),('zero','+0.00000000E+00',0),('negative-zero','-0.00000000E+00',0),('exponent','+1.25000000E-09',1.25e-9)]:
 a.append(case('34401a',n,[call('DmmMeasureVoltageDc',range=None)],[q(':MEAS:VOLT:DC? DEF,DEF',body)],qty(value,'Volt')))
for n,body in [('short','1.25'),('unsigned','1.25000000E+00'),('small-exponent','+1.25000000E+0'),('lower-e','+1.25000000e+00'),('wrong-decimal','+12.5000000E+00'),('leading-space',' +1.25000000E+00'),('trailing-space','+1.25000000E+00 '),('csv','+1.25000000E+00,+2.50000000E+00'),('negative-sentinel','-9.90000000E+37'),('giant','+1.00000000E+30'),('nan','NaN'),('infinity','Infinity'),('binary','#215+1.25000000E+00'),('control','+1.25000000E+00\n')]:
 a.append(case('34401a','reject-'+n,[call('DmmMeasureVoltageDc',range=None)],[q(':MEAS:VOLT:DC? DEF,DEF',body)],error='Parse'))
for error in ['Transport','Timeout']:
 a.append(case('34401a',error.lower(),[call('DmmMeasureVoltageDc',range=None)],[{'kind':'query','message':':MEAS:VOLT:DC? DEF,DEF','error':error}],error=error))
save('34401a',a)

def counter_steps(kind='FREQ',channel=1,gate='0.100',body='+1.2500000000000E+06'):
 return [w(f':CONF:{kind} DEF,DEF,(@{channel})'),w(':SENS:FREQ:ARM:STAR:SOUR IMM'),w(':SENS:FREQ:ARM:STOP:SOUR TIM'),w(':SENS:FREQ:ARM:STOP:TIM '+gate),w(':FORM ASC'),q(':READ?',body)]
a=[]
for channel in [1,2]:
 for name,wire,body,value,unit in [('Frequency','FREQ','+1.2500000000000E+06',1250000,'Hertz'),('Period','PER','+8.0000000000000E-07',8e-7,'Second')]:
  calls=([call('CounterSelectChannel',channel=channel)]if channel==2 else[])+[call('CounterMeasure'+name)]
  a.append(case('53132a',f'{name}-channel-{channel}',calls,counter_steps(wire,channel,body=body),qty(value,unit)))
for seconds,wire in [(.001,'0.001'),(.029,'0.029'),(.1,'0.100'),(1000,'1000.000')]:
 a.append(case('53132a','gate-'+wire,[call('CounterSetGateTime',seconds=seconds),call('CounterMeasureFrequency')],counter_steps(gate=wire),qty(1250000,'Hertz')))
a.append(case('53132a','switch-intent-and-repeat',[call('CounterSetGateTime',seconds=.01),call('CounterSelectChannel',channel=2),call('CounterMeasureFrequency'),call('CounterSelectChannel',channel=1),call('CounterMeasurePeriod')],counter_steps(channel=2,gate='0.010')+counter_steps('PER',1,'0.010','+8.0000000000000E-07'),qty(8e-7,'Second')))
for n,v in [('zero',0),('negative',-1),('too-small',.000999),('off-grid',.0010000000000000002),('too-large',1000.001)]:
 a.append(case('53132a','invalid-gate-'+n,[call('CounterSetGateTime',seconds=v)],[],error='InvalidArgument'))
for channel in [0,3,4294967295]:a.append(case('53132a','invalid-channel-'+str(channel),[call('CounterSelectChannel',channel=channel)],[],error='InvalidArgument'))
for n,body,value in [('one-digit','1E0',1),('unsigned','1.25E+3',1250),('lower-e','1.25e3',1250),('max-significant','1.23456789012345E+0',1.23456789012345),('leading-zeros','0001.25E+0',1.25)]:
 a.append(case('53132a','parse-'+n,[call('CounterMeasureFrequency')],counter_steps(body=body),qty(value,'Hertz')))
for n,body,error in [('nan-sentinel','+9.91E+37','Device'),('zero','0E0','Device'),('negative','-1.25E3','Device'),('underflow','1E-999','Device'),('no-exponent','1.25','Parse'),('sixteen-significant','1.234567890123456E+0','Parse'),('leading-space',' 1E0','Parse'),('trailing-space','1E0 ','Parse'),('no-frac','1.E0','Parse'),('no-leading','.1E0','Parse'),('unit','1E0HZ','Parse'),('csv','1E0,2E0','Parse'),('nan','NaN','Parse'),('infinity','Inf','Parse'),('overflow','1E999','Parse'),('binary','#131E0','Parse')]:
 a.append(case('53132a','reject-'+n,[call('CounterMeasureFrequency')],counter_steps(body=body),error=error))
for i,step in enumerate(counter_steps()):
 for error in ['Transport','Timeout']:
  script=counter_steps()[:i]+[{k:v for k,v in step.items()if k!='response'}|{'error':error}]
  a.append(case('53132a',f'failure-step-{i}-{error}',[call('CounterMeasureFrequency')],script,error=error))
save('53132a',a)

def guards():return [q(':FUNC:SHAP?','SIN'),q(':OUTP:LOAD?','+5.000000E+01'),q(':VOLT:UNIT?','VPP')]+[q(':'+s+':STAT?','0')for s in ['AM','FM','BM','FSK','SWE']]
a=[]
for hz,wire in [(.0001,'1.00000000000E-04'),(1000,'1.00000000000E+03'),(15000000,'1.50000000000E+07')]:
 a.append(case('33120a','frequency-'+str(hz),[call('FunctionGeneratorSetFrequency',hz=hz)],guards()+[w(':FREQ '+wire)],{'kind':'unit'}))
for vpp,wire,offset in [(.05,'5.00000000000E-02','+0.000000E+00'),(2,'2.00000000000E+00','+4.000000E+00'),(10,'1.00000000000E+01','+0.000000E+00')]:
 a.append(case('33120a','amplitude-'+str(vpp),[call('FunctionGeneratorSetAmplitude',vpp=vpp)],guards()+[q(':VOLT:OFFS?',offset),w(':VOLT '+wire)],{'kind':'unit'}))
for offset,wire,amp in [(-4,'-4.00000000000E+00','+2.000000E+00'),(0,'0.00000000000E+00','+1.000000E+00'),(4,'4.00000000000E+00','+2.000000E+00')]:
 a.append(case('33120a','offset-'+str(offset),[call('FunctionGeneratorSetOffset',volts=offset)],guards()+[q(':VOLT?',amp),w(':VOLT:OFFS '+wire)],{'kind':'unit'}))
a.append(case('33120a','read-frequency',[call('FunctionGeneratorReadFrequency')],guards()+[q(':FREQ?','+1.000000000000E+03')],qty(1000,'Hertz')))
for op,arg,values in [('FunctionGeneratorSetFrequency','hz',[0,-1,.0000999,15000001]),('FunctionGeneratorSetAmplitude','vpp',[0,-1,.049999,10.001]),('FunctionGeneratorSetOffset','volts',[-4.000001,4.000001])]:
 for i,v in enumerate(values):a.append(case('33120a',f'{op}-invalid-{i}',[call(op,**{arg:v})],[],error='InvalidArgument'))
for i,wrong,error in [(0,'SQU','Device'),(0,'PULS','Parse'),(1,'+1.000000E+06','Device'),(1,'0','Parse'),(2,'VRMS','Device'),(2,'VOLTS','Parse')]+[(i,'1','Device')for i in range(3,8)]+[(i,'ON','Parse')for i in range(3,8)]:
 script=guards()[:i+1];script[-1]['response']=wrong
 a.append(case('33120a',f'guard-{i}-{wrong}',[call('FunctionGeneratorSetFrequency',hz=1000)],script,error=error))
for n,op,args,query,reply in [('amplitude-rail','FunctionGeneratorSetAmplitude',{'vpp':3},':VOLT:OFFS?','+3.750000E+00'),('offset-small-amplitude','FunctionGeneratorSetOffset',{'volts':1},':VOLT?','+1.000000E-01'),('rounded-wire-rail','FunctionGeneratorSetAmplitude',{'vpp':2.9999999999996},':VOLT:OFFS?','3.5000000000001')]:
 a.append(case('33120a','coupled-'+n,[call(op,**args)],guards()+[q(query,reply)],error='InvalidArgument'))
for body,error,n in [('-1','Device','negative'),('0','Device','zero'),('15000001','Device','above-max'),('1E30','Parse','giant'),('1,2','Parse','csv'),(' 1E3','Parse','leading-space'),('1E3 ','Parse','trailing-space'),('NaN','Parse','nan'),('1.E3','Parse','missing-fraction')]:
 a.append(case('33120a','read-reject-'+n,[call('FunctionGeneratorReadFrequency')],guards()+[q(':FREQ?',body)],error=error))
for i,step in enumerate(guards()+[w(':FREQ 1.00000000000E+03')]):
 for error in ['Transport','Timeout']:
  script=(guards()+[w(':FREQ 1.00000000000E+03')])[:i]+[{k:v for k,v in step.items()if k!='response'}|{'error':error}]
  a.append(case('33120a',f'failure-step-{i}-{error}',[call('FunctionGeneratorSetFrequency',hz=1000)],script,error=error))
save('33120a',a)
