"""Dependency-free R1E validator for the inventory and committed oracle."""
import csv, hashlib, json, sys
from collections import Counter
def decode(s):
    out=[]; i=0
    while i<len(s):
        if s[i]!='\\': out.append(s[i]); i+=1; continue
        i+=1
        if i>=len(s): out.append('\\'); break
        c=s[i]; i+=1
        if c in 'nrt': out.append({'n':'\n','r':'\r','t':'\t'}[c])
        elif c=='\\': out.append('\\')
        elif c=='u': out.append(chr(int(s[i:i+4],16))); i+=4
        else: out.append(c)
    return ''.join(out)
inv=list(csv.DictReader(open(sys.argv[1]),delimiter='\t'))
document=json.load(open(sys.argv[2])); assert document['schema']=='AKNL-URL-P1A-R1E'; out=document['records']
expected={'KURL-001':5,'KURL-002':12,'KURL-003':8,'KURL-004':6,'KURL-005':12,'URL257-001':6,'RLINK-001':10,'SEC-001':6,'KURL-006':8,'KURL-007':6,'KURL-008':6,'KURL-009':8,'KURL-010':12,'KURL-011':10,'KURL-012':8,'KURL-013':10,'KURL-014':6,'KURL-015':10,'KURL-016':6,'KURL-017':12,'URL257-002':12,'RLINK-002':11}
assert len(inv)==len(out)==190 and Counter(x['group'] for x in inv)==Counter(x['group'] for x in out)==expected
for i,(x,y) in enumerate(zip(inv,out)):
    assert (x['id'],x['group'],x['stable_source_test_name'])==(y['id'],y['group'],y['source_test'])
    e=x['escaped_input']; canon=f"{e}|{x['base']}|{x['operation']}" if x['operation']!='repository_link' else f"{x['allowed_schemes_supplied']}|{x['allowed_schemes']}|{x['default_scheme_supplied']}|{x['default_scheme']}|{x['repo_link_supplied']}|{e}|{x['operation']}"
    assert x['id'].endswith(hashlib.sha256(canon.encode()).hexdigest()[:12]) and y['input_utf8_hex']==decode(x['escaped_input']).encode().hex()
    k,u=y['kernel'],y['url257']; assert ('error_kind' in k and 'error_audience' in k) if not k['success'] else all(q in k for q in ('value','scheme_accessor','query_pairs','fragment_accessor'))
    assert ('parse_error_category' in u and u['parse_error_category']!='PARSE_FAILURE') if not u['success'] else all(q in u for q in ('host_kind','explicit_port','query_pairs','fragment_present'))
    if x['group']=='KURL-014': assert y['comparison'] is not None
    if x['group']=='SEC-001':
        s=y['security']; keys={'kernel_error_applicable','kernel_error_username_canary','kernel_error_password_canary','kernel_error_query_canary','kernel_error_fragment_canary','kernel_debug_applicable','kernel_debug_username_canary','kernel_debug_password_canary','kernel_debug_query_canary','kernel_debug_fragment_canary','repository_link_applicable','repository_link_error_applicable','repository_link_display_applicable','repository_link_username_canary','repository_link_password_canary','repository_link_query_canary','repository_link_fragment_canary','url_display_applicable','repository_link'}
        assert set(s)==keys and s['url_display_applicable'] is False and s['repository_link_applicable'] is True
        active={'credential':{'username','password'},'query':{'query'},'fragment':{'fragment'}}
        cls=next(k for k in active if k in x['stable_source_test_name']); invalid=x['stable_source_test_name'].startswith('invalid_')
        expected_apps={'kernel_error_applicable':invalid,'kernel_debug_applicable':not invalid,'repository_link_applicable':True,'repository_link_error_applicable':invalid,'repository_link_display_applicable':not invalid,'url_display_applicable':False}
        assert all(type(s[k]) is bool and s[k]==v for k,v in expected_apps.items())
        for prefix,app in (('kernel_error',invalid),('kernel_debug',not invalid),('repository_link',invalid)):
            if prefix=='repository_link': app = s['repository_link_error_applicable'] if invalid else s['repository_link_display_applicable']
            for field in ('username','password','query','fragment'):
                value=s[f'{prefix}_{field}_canary']
                assert value is None if not app else type(value) is bool and value==(field in active[cls])
        nested=s['repository_link']; assert nested['success']==(not invalid) and type(nested['success']) is bool
        success_keys={'success','default_scheme_applied','url_projection','display_username_canary','display_password_canary','display_query_canary','display_fragment_canary'}
        failure_keys={'success','error_kind','error_audience','error_username_canary','error_password_canary','error_query_canary','error_fragment_canary'}
        assert set(nested)==(failure_keys if invalid else success_keys)
        if invalid: assert nested['error_kind']=='INVALID_INPUT' and nested['error_audience']=='SYSTEM'
        else: assert type(nested['default_scheme_applied']) is bool
        for field in ('username','password','query','fragment'):
            nested_key=('error_' if invalid else 'display_')+field+'_canary'
            assert type(nested[nested_key]) is bool and s['repository_link_'+field+'_canary']==nested[nested_key]
rlink={'build_success':(True,None,None),'apply_default_scheme_success':(True,None,None),'apply_default_scheme_with_scheme_seperator_only_success':(True,None,None),'malformed_repo_link_error':(False,'INVALID_INPUT','SYSTEM'),'no_allowed_schema_provided_error':(False,'INVALID_INPUT','SYSTEM'),'no_default_scheme_provided_error':(False,'INVALID_INPUT','USER'),'empty_default_scheme_error':(False,'INVALID_INPUT','SYSTEM'),'invalid_default_scheme_not_in_allowed_error':(False,'INVALID_INPUT','SYSTEM'),'no_repo_link_error':(False,'INVALID_INPUT','SYSTEM'),'repo_link_whitespace_error':(False,'INVALID_INPUT','SYSTEM'),'unallowed_repo_link_scheme_error':(False,'INVALID_INPUT','SYSTEM')}
for y in out:
 if y['group']=='RLINK-002':
  ok,k,a=rlink[y['source_test']]; r=y['repository_link']; assert r['success']==ok and (ok or (r['error_kind'],r['error_audience'])==(k,a))
print('PASS inventory=190 output=190 order=PASS ids=PASS RLINK-002=11 SEC-001=6 projections=PASS')
