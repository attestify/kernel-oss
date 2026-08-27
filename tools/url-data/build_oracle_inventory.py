"""Build the explicit-output-path AKNL-URL-P1A-R1E oracle inventory."""
import hashlib
import sys
from pathlib import Path

G = {
"KURL-001": [("create_new_url_success","http://www.example.com:8080/some/path?query=value#the-fragment"),("create_new_url_only_scheme_and_host_success","https://www.example.com"),("create_new_url_multiple_queries_success","http://www.example.com:8080/some/path?query1=value1&query2=value2#fragment"),("accessors_success","https://example.com/path?one=1&two=2#frag"),("create_from_bad_url_input_error","Some-Bad-Input")],
"KURL-002": [("line-preprocessing",x) for x in ["\thttps://example.test/","\nhttps://example.test/","\rhttps://example.test/","https://example.test/\n","https://example.test/\t"," https://example.test/","\u00a0https://example.test/","https://example.test/ a","https://example.test/%0A","https://example.test/%09","https://example.test/\r\n","https://example.test/\u2003"]],
"KURL-003": [("query-fragment-absence",x) for x in ["https://e.test/","https://e.test/?","https://e.test/#","https://e.test/?#","https://e.test/?a","https://e.test/#a","https://e.test/?a=","https://e.test/?#a"]],
"KURL-004": [("port-state",x) for x in ["https://e.test/","https://e.test:443/","http://e.test:80/","https://e.test:0/","https://e.test:65535/","https://e.test:99999/"]],
"KURL-005": [("query-pair-semantics",x) for x in ["https://e.test/?a=1&b=2","https://e.test/?a=1&a=2","https://e.test/?=v","https://e.test/?a=","https://e.test/?a","https://e.test/?a=b=c","https://e.test/?a+b=c+d","https://e.test/?%61=%62","https://e.test/?x=%FF","https://e.test/?x=%C3%28","https://e.test/?&&","https://e.test/?a=1&=2"]],
"URL257-001": [("synthetic-credential",x) for x in ["https://user:canary@example.test/","https://user@example.test/","https://:canary@example.test/","https://user:canary@e.test/","https://u%40x:canary@example.test/","https://user:canary@e.test/a"]],
"SEC-001": [("valid_credential_canary","https://canaryuser:canarypass@example.test/"),("invalid_credential_canary","https://canaryuser:canarypass@[bad]/"),("valid_query_canary","https://e.test/?token=canaryquery"),("invalid_query_canary","https://e.test:99999/?token=canaryquery"),("valid_fragment_canary","https://e.test/#canaryfragment"),("invalid_fragment_canary","https://[:::1]/#canaryfragment")],
"KURL-006": [("scheme-class",x) for x in ["http://e.test/","https://e.test/","ws://e.test/","wss://e.test/","ftp://e.test/","foo://e.test/","mailto:x@y.test","data:text/plain,x"]],
"KURL-007": [("path-class",x) for x in ["foo:bar","mailto:a@b","https://e.test/a","file:///a","data:,x","foo:/a"]],
"KURL-008": [("file-url",x) for x in ["file:///tmp/a","file://localhost/tmp/a","file:///C:/a","file://server/share/a","file:///","file:///a%20b"]],
"KURL-009": [("idna-domain",x) for x in ["https://bücher.example/","https://faß.de/","https://xn--bcher-kva.example/","https://例え.テスト/","https://é.example/","https://☃.net/","https://δοκιμή.gr/","https://e.test/"]],
"KURL-010": [("ipv4",x) for x in ["http://127.0.0.1/","http://2130706433/","http://0177.0.0.1/","http://0x7f.0.0.1/","http://127.1/","http://127.0.1/","http://256.0.0.1/","http://9999999999/","http://0xffffffff/","http://0x100000000/","http://1.2.3.4/","http://01.02.03.04/"]],
"KURL-011": [("ipv6",x) for x in ["http://[::1]/","http://[2001:db8::1]/","http://[::ffff:192.0.2.1]/","http://[2001:db8:0:0:0:0:2:1]/","http://[2001:db8::2:1]/","http://[::]/","http://[:::1]/","http://[2001:db8]/","http://[::ffff:999.1.1.1]/","http://[1:2:3:4:5:6:7:8:9]/"]],
"KURL-012": [("invalid-host-port",x) for x in ["http://exa mple/","http://[bad]/","http://%/","http://e.test:abc/","http://e.test:-1/","http:///a","https://","http://?a"]],
"KURL-013": [("percent-dot",x) for x in ["https://e.test/%","https://e.test/%2F","https://e.test/%2e/","https://e.test/a/../b","https://e.test/a/%2e%2e/b","https://e.test/%3F?q","https://e.test/%23","https://e.test/?x=%","https://e.test/a%2Fb","https://e.test/%00"]],
"KURL-014": [("EQ-001","https://e.test/"),("EQ-001","HTTPS://E.TEST/"),("EQ-002","https://e.test:443/"),("EQ-002","https://e.test/"),("EQ-003","http://127.0.0.1/"),("EQ-003","http://2130706433/")],
"KURL-015": [("authority-credentials-backslash",x) for x in ["https://u:p@e.test/","https://u:p@@e.test/","http:\\e.test\\a","https://u@e.test/","https://@e.test/","https://u%40x@e.test/","http://e.test\\a","https://u:p@e.test/a","https://u:p@e.test/?q","https://u:p@e.test/#f"]],
"KURL-016": [("malformed-or-scheme-only",x) for x in [""," ","http:","https:","://","not a url"]],
"KURL-017": [("form-query",x) for x in ["https://e.test/?a+b=c+d","https://e.test/?a=1&a=2","https://e.test/?=v","https://e.test/?a=","https://e.test/?a=b=c","https://e.test/?x=%FF","https://e.test/?x=%C3%28","https://e.test/?+","https://e.test/?a&&b","https://e.test/?a=1&","https://e.test/?%2B=%2B","https://e.test/?first=eq=tail"]],
"URL257-002": [("url257-diagnostic-category",x) for x in ["not a url","http://[bad]/","http://e.test:abc/","https://","http://%/","http://e.test:99999/","http:///","http://[:::1]/"," ","http://exa mple/","http://?a","http://[1:2:3:4:5:6:7:8:9]/"]],
}
R2=[("build_success","file,git,https","git","true","true","https://github.com/nape/processes/rust-ci"),("apply_default_scheme_success","file,git","git","true","true","github.com/nape/processes/rust-ci"),("apply_default_scheme_with_scheme_seperator_only_success","file,git","git","true","true","://github.com/nape/processes/rust-ci"),("malformed_repo_link_error","file,git","git","true","true",":/github.com/nape/processes/rust-ci"),("no_allowed_schema_provided_error","","git","false","true","https://github.com/nape/processes/rust-ci"),("no_default_scheme_provided_error","file,git,https","","true","false","https://github.com/nape/processes/rust-ci"),("empty_default_scheme_error","file,git,https","","true","true","https://github.com/nape/processes/rust-ci"),("invalid_default_scheme_not_in_allowed_error","file,git,https","ssh","true","true","https://github.com/nape/processes/rust-ci"),("no_repo_link_error","file,git,https","git","true","true","false",""),("repo_link_whitespace_error","file,git,https","git","true","true","    "),("unallowed_repo_link_scheme_error","file,git,https","git","true","true","ssh://localhost")]
R1=[(f"rlink-synthetic-{i:02d}","file,git,https","git","true","true","true",x) for i,x in enumerate(["https://example.test/r","git://example.test/r","file:///tmp/r","example.test/r","ssh://example.test/r","","   ","https://","git://","ftp://example.test/r"],1)]
def esc(s): return s.encode("unicode_escape").decode("ascii")
rows=[]; seen={}; ordinal=0
def add(group,name,value,op="parse",allowed="",default="",allowed_supplied="false",default_supplied="false",present="true",comparison="",security="synthetic"):
 global ordinal
 ordinal+=1; e=esc(value); canon=f"{e}|N/A|{op}" if op!="repository_link" else f"{allowed_supplied}|{allowed}|{default_supplied}|{default}|{present}|{e}|{op}"; ident=f"legacy-{group.lower()}-{ordinal:06d}-{hashlib.sha256(canon.encode()).hexdigest()[:12]}"
 key=(e,"N/A",op); duplicate="unique" if key not in seen else f"reused for distinct behavior: {seen[key]} versus {group}"; seen.setdefault(key,group)
 citation=f"src/values/specification/repository_link/tests.rs::{name}" if group.startswith("RLINK") else (f"src/values/uri/url/tests.rs::{name}" if group=="KURL-001" else "Attestify behavior-specific synthetic case")
 rows.append([ident,group,name,e,"N/A",op,"kernel-fields;kernel-accessors;url257-projection",allowed,default,allowed_supplied,default_supplied,present,comparison,security,citation,duplicate])
for group, xs in G.items():
 for name,value in xs: add(group,name,value,comparison=name if group=="KURL-014" else "",security="canary" if group=="SEC-001" else "synthetic")
for group,xs in (("RLINK-001",R1),("RLINK-002",R2)):
 for entry in xs:
  if len(entry)==6:
   n,a,d,asup,dsup,v=entry; p="false" if n=="no_repo_link_error" else "true"
  else:
   n,a,d,asup,dsup,p,v=entry
  add(group,n,v,"repository_link",a,d,asup,dsup,p,security="synthetic")
assert len(rows)==190
h="id\tgroup\tstable_source_test_name\tescaped_input\tbase\toperation\texpected_projection_fields\tallowed_schemes\tdefault_scheme\tallowed_schemes_supplied\tdefault_scheme_supplied\trepo_link_supplied\tcomparison_class_id\tsecurity_classification\tsource_citation\tduplicate_reuse_status_reason"
if len(sys.argv)!=2: raise SystemExit("usage: build_oracle_inventory.py OUTPUT_PATH")
Path(sys.argv[1]).write_text(h+"\n"+"\n".join("\t".join(x) for x in rows)+"\n")
