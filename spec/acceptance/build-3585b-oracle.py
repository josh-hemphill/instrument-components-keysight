"""Independent native HP3585B acceptance from manual diagrams/spec, no production reads."""
import json
from pathlib import Path
root=Path(__file__).parent

def call(op,**args):return {'operation':op,'arguments':args}
def q(message,response):return {'kind':'query','message':message,'response':response}
def w(message):return {'kind':'write','message':message}
def c(name,calls,script,value=None,error=None):
 v={'id':'keysight.hp3585b|E|'+name,'calls':calls,'script':script+[{'kind':'close'}]}
 v['expectedError'if error else'expected']=error or value;return v
unit={'kind':'unit'};rows=[]
for op,prefix,vals in [('SpectrumAnalyzerSetCenterFrequency','CF',[(1,'1'),(10000000,'10000000'),(40100000,'40100000')]),('SpectrumAnalyzerSetSpan','FS',[(1,'1'),(5000,'5000'),(40100000,'40100000')]),('SpectrumAnalyzerSetRbw','RB',[(x,str(x))for x in[3,10,30,100,300,1000,3000,10000,30000]]),('SpectrumAnalyzerSetVbw','VB',[(x,str(x))for x in[1,3,10,30,100,300,1000,3000,10000,30000]])]:
 for value,wire in vals:rows.append(c(op+'-'+wire,[call(op,hz=value)],[w(prefix+wire+'HZ')],unit))
for value,wire in[(-70,'-70.0'),(-20.1,'-20.1'),(-0.0,'0.0'),(40,'40.0')]:rows.append(c('reference-'+wire,[call('SpectrumAnalyzerSetRefLevel',dbm=value)],[w('RL'+wire+'DM')],unit))
for op,values in [('SpectrumAnalyzerSetCenterFrequency',[0,-1,1.5,40100001]),('SpectrumAnalyzerSetSpan',[0,-1,1.5,40100001]),('SpectrumAnalyzerSetRbw',[0,1,2,5,30001]),('SpectrumAnalyzerSetVbw',[0,2,5,30001])]:
 for i,value in enumerate(values):rows.append(c(op+'-invalid-'+str(i),[call(op,hz=value)],[],error='InvalidArgument'))
for i,value in enumerate([-70.1,40.1,0.15,0.10000000000000002]):rows.append(c('reference-invalid-'+str(i),[call('SpectrumAnalyzerSetRefLevel',dbm=value)],[],error='InvalidArgument'))
rows.append(c('stored-peak',[call('SpectrumAnalyzerMarkerPeak')],[w('CL'),w('SP')],unit))
samples=[('ordinary','+  1000000.0,-  20.00E-0',1000000,-20),('zero','+        0.0,+   0.00E-0',0,0),('upper','+ 40100000.0,+  30.00E-0',40100000,30),('negative-zero','-        0.0,-   0.00E-0',0,0),('full-digit-syntax','+040100000.0,-0020.25E-0',40100000,-20.25)]
for name,body,x,y in samples:
 assert len(body)==24,(name,len(body))
 for axis,value,unitname in[('X',x,'Hertz'),('Y',y,'Dbm')]:rows.append(c('marker-'+axis+'-'+name,[call('SpectrumAnalyzerMarker'+axis)],[w('CL'),q('D2,T6',body)],{'kind':'quantity','value':value,'unit':unitname,'status':'Valid'}))
base='+  1000000.0,-  20.00E-0'
bads=[('short',base[:-1]),('long',base+' '),('trailing-frame',base+'\r\n'),('csv',base+',0'),('leading-space',' '+base[1:]),('freq-no-sign','0'+base[1:]),('freq-tab',base[:1]+'\t'+base[2:]),('freq-internal-space','+  10 0000.0,-  20.00E-0'),('freq-no-point',base[:10]+'0'+base[11:]),('freq-negative','-  1000000.0,-  20.00E-0'),('freq-outside','+ 40100000.1,-  20.00E-0'),('comma','+  1000000.0;-  20.00E-0'),('amp-no-sign',base[:13]+' '+base[14:]),('amp-internal-space','+  1000000.0,- 2 0.00E-0'),('amp-no-point',base[:18]+'0'+base[19:]),('amp-lower-e',base.replace('E-0','e-0')),('amp-exponent',base.replace('E-0','E-1')),('amp-positive-exp',base.replace('E-0','E+0')),('unicode',base.replace('20','２０')),('binary','#224'+base)]
for name,body in bads:
 for axis in['X','Y']:rows.append(c('marker-'+axis+'-reject-'+name,[call('SpectrumAnalyzerMarker'+axis)],[w('CL'),q('D2,T6',body)],error='Parse'))
for op,script in [('SpectrumAnalyzerSetCenterFrequency',[w('CF10000000HZ')]),('SpectrumAnalyzerSetSpan',[w('FS5000HZ')]),('SpectrumAnalyzerSetRbw',[w('RB30HZ')]),('SpectrumAnalyzerSetVbw',[w('VB10HZ')]),('SpectrumAnalyzerSetRefLevel',[w('RL-20.0DM')]),('SpectrumAnalyzerMarkerPeak',[w('CL'),w('SP')]),('SpectrumAnalyzerMarkerX',[w('CL'),q('D2,T6',base)]),('SpectrumAnalyzerMarkerY',[w('CL'),q('D2,T6',base)])]:
 args={'hz':10000000}if op.endswith('CenterFrequency')else{'hz':5000}if op.endswith('Span')else{'hz':30}if op.endswith('Rbw')else{'hz':10}if op.endswith('Vbw')else{'dbm':-20}if op.endswith('RefLevel')else{}
 for i,step in enumerate(script):
  for error in['Transport','Timeout']:
   failed={k:v for k,v in step.items()if k!='response'}|{'error':error}
   rows.append(c(op+'-failure-'+str(i)+'-'+error,[call(op,**args)],script[:i]+[failed],error=error))
assert len({x['id']for x in rows})==len(rows)
(root/'3585b.catalog-cases.json').write_text(json.dumps({'schemaVersion':1,'contractMajor':1,'description':'Independent own-manual diagram-derived SYNTHETIC native traffic; no observed hardware transcript. CL-only marker prelude; stored peak semantics.','cases':rows},indent=2)+'\n')
print('HP3585B',len(rows),'independent synthetic software cases')
