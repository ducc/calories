def try_fromjson: if . == null or .[0:1] != "{" then . else fromjson end; 
def try_frombase64: if . == null then . else @base64d end;

[.[] | {
  method: .method, 
  host: .host, 
  path: .path, 
  query: .query,
  request: { 
    # headers: .request.header.headers, 
	mimeType: .request.mimeType, 
	contentEncoding: .request.contentEncoding, 
	body: .request.body.text | try_fromjson
  }, 
  response: {
    mimeType: .response.mimeType, 
    encoding: .response.body.encoding,
	headers: .response.header.headers, 
	body: (if .response.body.encoding == "base64" then (.response.body.encoded | try_frombase64) else (.response.body.text | try_fromjson) end),
  }
}]