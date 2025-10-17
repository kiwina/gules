# https://developers.google.com/jules/api/reference/rest/v1alpha/sources

  * Resource: Source  
    * JSON representation
  * GitHubRepo
    * JSON representation
  * GitHubBranch
    * JSON representation
  * Methods



## Resource: Source

An input source of data for a session.

JSON representation  
---  
      
    
    {
      "name": string,
      "id": string,
    
      // Union field source can be only one of the following:
      "githubRepo": {
        object ([GitHubRepo](/jules/api/reference/rest/v1alpha/sources#GitHubRepo))
      }
      // End of list of possible types for union field source.
    }  
  
Fields  
---  
`name` |  `string` Identifier. The full resource name (e.g., "sources/{source}").  
`id` |  `string` Output only. The id of the source. This is the same as the "{source}" part of the resource name (e.g., "sources/{source}").  
Union field `source`. The input data source. `source` can be only one of the following:  
`githubRepo` |  `object (`[GitHubRepo](/jules/api/reference/rest/v1alpha/sources#GitHubRepo)`)` A GitHub repo.  
  
## GitHubRepo

A GitHub repo.

JSON representation  
---  
      
    
    {
      "owner": string,
      "repo": string,
      "isPrivate": boolean,
      "defaultBranch": {
        object ([GitHubBranch](/jules/api/reference/rest/v1alpha/sources#GitHubBranch))
      },
      "branches": [
        {
          object ([GitHubBranch](/jules/api/reference/rest/v1alpha/sources#GitHubBranch))
        }
      ]
    }  
  
Fields  
---  
`owner` |  `string` The owner of the repo; the `<owner>` in `https://github.com/<owner>/<repo>`.  
`repo` |  `string` The name of the repo; the `<repo>` in `https://github.com/<owner>/<repo>`.  
`isPrivate` |  `boolean` Whether this repo is private.  
`defaultBranch` |  `object (`[GitHubBranch](/jules/api/reference/rest/v1alpha/sources#GitHubBranch)`)` The default branch for this repo.  
`branches[]` |  `object (`[GitHubBranch](/jules/api/reference/rest/v1alpha/sources#GitHubBranch)`)` The list of active branches for this repo.  
  
## GitHubBranch

A GitHub branch.

JSON representation  
---  
      
    
    {
      "displayName": string
    }  
  
Fields  
---  
`displayName` |  `string` The name of the GitHub branch.  
  
## Methods  
  
---  
  
### `[get](/jules/api/reference/rest/v1alpha/sources/get)`

|  Gets a single source.  
  
### `[list](/jules/api/reference/rest/v1alpha/sources/list)`

|  Lists sources.
