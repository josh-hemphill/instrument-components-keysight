"""Add the independently reviewed own-manual HighZ fixture without altering v1.

Input is the frozen independent v1 oracle, never a production profile/backend.
Source: 33120-90006 printed151/PDF153, positive9.9E37 means high impedance.
"""
import json
from pathlib import Path
root=Path(__file__).parent
value=json.loads((root/'33120a.catalog-cases.json').read_text())
case={'id':'keysight.33120a|E|guard-load-high-impedance-sentinel','calls':[{'operation':'FunctionGeneratorSetFrequency','arguments':{'hz':1000}}],'script':[{'kind':'query','message':':FUNC:SHAP?','response':'SIN'},{'kind':'query','message':':OUTP:LOAD?','response':'+9.900000E+37'},{'kind':'close'}],'expectedError':'Device'}
assert not any(c['id']==case['id']for c in value['cases'])
value['cases'].append(case)
value['description']+=' v2 adds own-manual HighZ sentinel policy.'
(root/'33120a-v2.catalog-cases.json').write_text(json.dumps(value,indent=2)+'\n')
print('PASS33120A v2 independent oracle:',len(value['cases']),'cases')
