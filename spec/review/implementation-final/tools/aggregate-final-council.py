"""Validate all independent ballots before computing per-model/category medians."""
import argparse,hashlib,json,statistics
from decimal import Decimal
from pathlib import Path
parser=argparse.ArgumentParser();parser.add_argument('directory',type=Path);args=parser.parse_args();root=args.directory
models=['HP5334A','HP5334B','HP3497A','HP8920A','HP8920B','HP8921A']
categories=['EVIDENCE','API','PROTOCOL','PARITY','VALIDATION','DELIVERY']
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
source_sha=sha(root/'source-manifest.json');evidence_sha=sha(root/'final-evidence-manifest.json');ballots=[]
for member in 'ABC':
    path=root/f'member-{member.lower()}-final.json';value=json.loads(path.read_text())
    assert value['member']==member and value['model']=='gpt-6.1-sol' and value['reasoning']=='high' and value['rubric']=='v1'
    assert value['sourceManifestSha256']==source_sha and value['evidenceManifestSha256']==evidence_sha
    assert set(value['scores'])==set(models)
    for model in models:
        assert set(value['scores'][model])==set(categories)
        for category in categories:
            score=value['scores'][model][category]
            assert isinstance(score,(int,float)) and 0<=score<=10,(member,model,category,score)
            assert Decimal(str(score))==Decimal(str(score)).quantize(Decimal('.1'))
    assert not value['mandatoryDefects'],(member,value['mandatoryDefects'])
    ballots.append(value)
rows=[];passed=True
for model in models:
    for category in categories:
        scores=[Decimal(str(b['scores'][model][category])) for b in ballots]
        median=statistics.median(scores);votes=sum(s>=Decimal('9.2') for s in scores);ok=median>=Decimal('9.2') and votes>=2;passed &= ok
        rows.append({'model':model,'category':category,'target':'>=9.2','scores':dict(zip('ABC',map(float,scores))),'median':float(median),'passVotes':votes,'status':'PASS' if ok else 'FAIL','range':[float(min(scores)),float(max(scores))]})
result={'rubric':'v1','round':'implementation-r6','model':'gpt-6.1-sol','reasoning':'high','sourceManifestSha256':source_sha,'evidenceManifestSha256':evidence_sha,'allCategoriesPass':passed,'rows':rows,'mandatoryDefects':[],'ballots':[{'member':b['member'],'sha256':sha(root/f'member-{b["member"].lower()}-final.json')} for b in ballots]}
(root/'result.json').write_text(json.dumps(result,indent=2)+'\n')
lines=['# HP implementation Council — final review','',f'Rubricv1; gpt-6.1-sol/high; three independent members; every category must satisfy >=9.2. Round implementation-r6. Source manifest SHA256 `{source_sha}`; evidence manifest SHA256 `{evidence_sha}`.','', 'All36 model/category grades pass; no unresolved mandatory defect.' if passed else 'INCOMPLETE: one or more category grades fail.','', '| Model | Category | Target | A | B | C | Median | Votes | Status |','|---|---|---|---:|---:|---:|---:|---:|---|']
for r in rows:lines.append(f'| {r["model"]} | {r["category"]} | >=9.2 | {r["scores"]["A"]:.1f} | {r["scores"]["B"]:.1f} | {r["scores"]["C"]:.1f} | {r["median"]:.1f} | {r["passVotes"]}/3 | {r["status"]} |')
lines += ['', 'See all independent Markdown/JSON ballots for actual evidence, confidence, minor findings and limitations. Medians are review judgments, not hardware proof. Public0.2.0-alpha.1 does not contain the additions; no release/merge or hardware validation is claimed.','']
(root/'RESULT.md').write_text('\n'.join(lines))
print('PASS all36 model/category Council medians' if passed else 'FAIL: Council targets not reached')
