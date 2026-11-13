"""Independent own-manual/spec module oracle; never reads production files."""
import json
from pathlib import Path
root=Path(__file__).parent

def call(op,**args):return {'operation':op,'arguments':args}
def q(msg,body):return {'kind':'query','message':msg,'response':body}
def w(msg):return {'kind':'write','message':msg}
def c(name,calls,script,value=None,error=None):
 out={'id':'keysight.34980a|E|'+name,'calls':calls,'script':script+[{'kind':'close'}]}
 out['expectedError'if error else'expected']=error or value
 return out
def save(model,rows):
 out={'schemaVersion':1,'contractMajor':1,'description':'Independent primary-manual/spec traffic, synthetic software tests, no hardware observations. Shared fixture mapping in fixture-inventory.json.','cases':rows}
 (root/(model+'.catalog-cases.json')).write_text(json.dumps(out,indent=2)+'\n');print(model,len(rows))
fixture={'schemaVersion':1,'driverId':'keysight.34980a','endpointScheme':'scpi','endpointAddress':'fixture','modules':[{'slot':1,'model':'34937A'},{'slot':2,'model':'34922A'},{'slot':3,'model':'34950A'}],'switches':[{'channel':x,'slot':1,'local':y}for x,y in[(7,1),(13,28),(29,29),(61,32)]]+[{'channel':x,'slot':2,'local':y}for x,y in[(101,1),(135,35),(136,36),(170,70)]],'digitalPorts':[{'port':7,'slot':3,'local':101,'direction':'Input'},{'port':13,'slot':3,'local':104,'direction':'Output'},{'port':29,'slot':3,'local':201,'direction':'Input'},{'port':61,'slot':3,'local':204,'direction':'Output'}],'commissioning':{'exclusiveMainframeNoScan':True,'noAutonomousActionsThroughoutLifetime':True,'normalPolarity':True,'allByteWidths':True,'handshakeHighImpedance':True,'memoryInputOutputDisabled':True,'inputThresholdVolts':2.5,'outputDrive':'Active','outputHighVolts':3.3}}
(root/'fixture-inventory.json').write_text(json.dumps(fixture,indent=2)+'\n')
for model,paths in [('34937a',[(7,1001),(13,1028),(29,1029),(61,1032)]),('34922a',[(101,2001),(135,2035),(136,2036),(170,2070)])]:
 rows=[]
 for logical,address in paths:
  for op,wire in [('SwitchCloseChannel','CLOS'),('SwitchOpenChannel','OPEN')]:
   rows.append(c(f'{model}-{op}-{logical}',[call(op,channel=logical)],[w(f':ROUT:{wire} (@{address})')],{'kind':'unit'}))
  for state in[False,True]:rows.append(c(f'{model}-state-{logical}-{state}',[call('SwitchChannelIsClosed',channel=logical)],[q(f':ROUT:CLOS? (@{address})','1'if state else'0')],{'kind':'boolean','value':state}))
 logical,address=paths[0]
 for i,body in enumerate(['ON','+1','1.0','1,0',' 1','1 ','','2','#11','１']):rows.append(c(f'{model}-state-malformed-{i}',[call('SwitchChannelIsClosed',channel=logical)],[q(f':ROUT:CLOS? (@{address})',body)],error='Parse'))
 for op,step in [('SwitchCloseChannel',w(f':ROUT:CLOS (@{address})')),('SwitchOpenChannel',w(f':ROUT:OPEN (@{address})')),('SwitchChannelIsClosed',q(f':ROUT:CLOS? (@{address})','1'))]:
  for error in['Transport','Timeout']:
   failed={k:v for k,v in step.items()if k!='response'}|{'error':error}
   rows.append(c(f'{model}-{op}-{error}',[call(op,channel=logical)],[failed],error=error))
 if model=='34922a':rows.append(c('34922a-same-bank-nonexclusive',[call('SwitchCloseChannel',channel=101),call('SwitchCloseChannel',channel=135),call('SwitchChannelIsClosed',channel=101)],[w(':ROUT:CLOS (@2001)'),w(':ROUT:CLOS (@2035)'),q(':ROUT:CLOS? (@2001)','1')],{'kind':'boolean','value':True}))
 save(model,rows)

def dg(address,direction):
 bank=(address//1000)*1000+(101 if(address%1000)<200 else 201)
 out=[q(f':CONF:DIG:WIDT? (@{bank+i})','1')for i in range(4)]
 out +=[q(f':CONF:DIG:DIR? (@{address})','INP'if direction=='Input'else'OUTP'),q(f':CONF:DIG:POL? (@{address})','NORM'),q(f':CONF:DIG:HAND:STAT? (@{bank})','HIMP'),q(f':SENS:DIG:MEM:ENAB? (@{bank})','0'),q(f':SOUR:DIG:MEM:ENAB? (@{bank})','0')]
 if direction=='Input':out+=[q(f':SENS:DIG:THR? (@{address})','+2.50000000E+00')]
 else:out+=[q(f':SOUR:DIG:DRIV? (@{address})','ACT'),q(f':SOUR:DIG:LEV? (@{address})','+3.30000000E+00')]
 return out
rows=[]
for port,address in[(7,3101),(29,3201)]:
 for value,body in[(0,'0'),(165,'165'),(255,'255'),(7,'007')]:rows.append(c(f'34950a-read-{port}-{body}',[call('DigitalIoReadByte',port=port)],dg(address,'Input')+[q(f':SENS:DIG:DATA:BYTE? DEC,(@{address})',body)],{'kind':'count','value':value}))
for port,address in[(13,3104),(61,3204)]:
 for value in[0,165,255]:rows.append(c(f'34950a-write-{port}-{value}',[call('DigitalIoWriteByte',port=port,value=value)],dg(address,'Output')+[w(f':SOUR:DIG:DATA:BYTE {value},(@{address})')],{'kind':'unit'}))
for port in[0,8,13,61]:rows.append(c(f'34950a-invalid-read-port-{port}',[call('DigitalIoReadByte',port=port)],[],error='InvalidArgument'))
for port in[0,8,7,29]:rows.append(c(f'34950a-invalid-write-port-{port}',[call('DigitalIoWriteByte',port=port,value=0)],[],error='InvalidArgument'))
for value in[256,4294967295]:rows.append(c(f'34950a-invalid-byte-{value}',[call('DigitalIoWriteByte',port=13,value=value)],[],error='InvalidArgument'))
for i,body in enumerate(['256','-1','+1','1.0','1E0','0xFF','#HFF','1,2',' 1','1 ','NaN','Inf','']):rows.append(c(f'34950a-invalid-byte-body-{i}',[call('DigitalIoReadByte',port=7)],dg(3101,'Input')+[q(':SENS:DIG:DATA:BYTE? DEC,(@3101)',body)],error='Parse'))
for direction,port,address,op,args in[('Input',7,3101,'DigitalIoReadByte',{}),('Output',13,3104,'DigitalIoWriteByte',{'value':165})]:
 steps=dg(address,direction)
 alternatives={0:[('2','Device'),('4','Device'),('BYTE','Parse')],1:[('2','Device')],2:[('4','Device')],3:[('2','Device')],4:[('OUTP'if direction=='Input'else'INP','Device'),('INPUT','Parse')],5:[('INV','Device'),('NORMAL','Parse')],6:[('OFF','Device'),('ON','Device'),('NONE','Parse')],7:[('1','Device'),('ON','Parse')],8:[('1','Device'),('OFF','Parse')]}
 if direction=='Input':alternatives[9]=[('+2.40000000E+00','Device'),('NaN','Parse')]
 else:alternatives[9]=[('OCOL','Device'),('ACTIVE','Parse')];alternatives[10]=[('+3.20000000E+00','Device'),('NaN','Parse')]
 for i,wrong_values in alternatives.items():
  for j,(body,error)in enumerate(wrong_values):
   script=dg(address,direction)[:i+1];script[-1]['response']=body
   rows.append(c(f'34950a-{direction}-guard-{i}-{j}',[call(op,port=port,**args)],script,error=error))
 last=q(f':SENS:DIG:DATA:BYTE? DEC,(@{address})','165')if direction=='Input'else w(f':SOUR:DIG:DATA:BYTE 165,(@{address})')
 steps+= [last]
 for i,step in enumerate(steps):
  for error in['Transport','Timeout']:
   script=steps[:i]+[{k:v for k,v in step.items()if k!='response'}|{'error':error}]
   rows.append(c(f'34950a-{direction}-failure-step-{i}-{error}',[call(op,port=port,**args)],script,error=error))
save('34950a',rows)
rows=[]
for op in['SwitchCloseChannel','SwitchOpenChannel','SwitchChannelIsClosed']:
 for channel in[0,999]:rows.append(c(f'34980a-invalid-{op}-{channel}',[call(op,channel=channel)],[],error='InvalidArgument'))
rows.append(c('34980a-shared-module-views',[call('SwitchCloseChannel',channel=7),call('DigitalIoReadByte',port=29),call('SwitchChannelIsClosed',channel=101)],[w(':ROUT:CLOS (@1001)')]+dg(3201,'Input')+[q(':SENS:DIG:DATA:BYTE? DEC,(@3201)','0'),q(':ROUT:CLOS? (@2001)','0')],{'kind':'boolean','value':False}))
save('34980a',rows)
