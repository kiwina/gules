# https://developers.google.com/jules/api/reference/rest/v1alpha/sources/list

  * HTTP request
  * Query parameters
  * Request body
  * Response body
    * JSON representation



Lists sources.

### HTTP request

`GET https://jules.googleapis.com/v1alpha/sources`

The URL uses [gRPC Transcoding](https://google.aip.dev/127) syntax.

### Query parameters

Parameters  
---  
`filter` |  `string` Optional. The filter expression for listing sources, based on AIP-160. If not set, all sources will be returned. Currently only supports filtering by name, which can be used to filter by a single source or multiple sources separated by OR.Example filters: - 'name=sources/source1 OR name=sources/source2'  
`pageSize` |  `integer` Optional. The number of sources to return. Must be between 1 and 100, inclusive. If unset, defaults to 30. If set to greater than 100, it will be coerced to 100.  
`pageToken` |  `string` Optional. A page token, received from a previous `sources.list` call.  
  
### Request body

The request body must be empty.

### Response body

Response message for the sources.list RPC.

If successful, the response body contains data with the following structure:

JSON representation  
---  
      
    
    {
      "sources": [
        {
          object ([Source](/jules/api/reference/rest/v1alpha/sources#Source))
        }
      ],
      "nextPageToken": string
    }  
  
Fields  
---  
`sources[]` |  `object (`[Source](/jules/api/reference/rest/v1alpha/sources#Source)`)` The sources from the specified request.  
`nextPageToken` |  `string` A token, which can be sent as `pageToken` to retrieve the next page. If this field is omitted, there are no subsequent pages.
