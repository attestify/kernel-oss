import csv, hashlib, sys
from collections import Counter
rows=list(csv.DictReader(open(sys.argv[1]),delimiter='\t'))
expected={'KURL-001':5,'KURL-002':12,'KURL-003':8,'KURL-004':6,'KURL-005':12,'URL257-001':6,'RLINK-001':10,'SEC-001':6,'KURL-006':8,'KURL-007':6,'KURL-008':6,'KURL-009':8,'KURL-010':12,'KURL-011':10,'KURL-012':8,'KURL-013':10,'KURL-014':6,'KURL-015':10,'KURL-016':6,'KURL-017':12,'URL257-002':12,'RLINK-002':11}
assert len(rows)==190 and Counter(x['group'] for x in rows)==expected
assert {x['stable_source_test_name'] for x in rows if x['group']=='KURL-001'}=={'create_new_url_success','create_new_url_only_scheme_and_host_success','create_new_url_multiple_queries_success','accessors_success','create_from_bad_url_input_error'}
assert {x['stable_source_test_name'] for x in rows if x['group']=='RLINK-002'}=={'build_success','apply_default_scheme_success','apply_default_scheme_with_scheme_seperator_only_success','malformed_repo_link_error','no_allowed_schema_provided_error','no_default_scheme_provided_error','empty_default_scheme_error','invalid_default_scheme_not_in_allowed_error','no_repo_link_error','repo_link_whitespace_error','unallowed_repo_link_scheme_error'}
ids=set()
for x in rows:
    escaped=x['escaped_input']; canon=f"{escaped}|{x['base']}|{x['operation']}"
    tail=hashlib.sha256(canon.encode()).hexdigest()[:12]
    assert x['id'].endswith('-'+tail) and x['id'] not in ids; ids.add(x['id'])
    if x['group'].startswith('RLINK'):
        assert x['allowed_schemes_supplied'] in ('true','false') and x['default_scheme_supplied'] in ('true','false') and x['repo_link_supplied'] in ('true','false')
    if x['group']=='KURL-014': assert x['comparison_class_id']
    if x['group']=='SEC-001': assert x['security_classification']=='canary'
    if x['duplicate_reuse_status_reason']=='': raise AssertionError('missing reuse status')
print('PASS rows=190 groups=22 ids=190 KURL-001=5 RLINK-002=11 conditional-fields=PASS')
