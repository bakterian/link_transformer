# Link Transformer

## Small rust library package for transforming URLs

This library can come handy when:
1) You work with internet web addresses while being outside of the private intranet network.  
  Those resources may still be accessible but the base URL has to be changed to a publicly exposed one.  
  
2) In the intranet you work with network drives which are not reachable from outside of the network.  
   Often there is an alternative web url that leads to this resource. This lib will do translation for you.  
   As a bonus in case hidden folders are used a dollar sign will be added to the chosen subfolders.  
  
3) The webservice that you like has a shorthand URL which is now no longer supported.  
   Thanks to this solution you won't need to memorize the full often difficult to remember web-domain address  
  
## How does it work 
The library simply replaces the base URL of the provided link with the matching URL found in a recipes.json file.  
If the base URL is unkown (not mentioned in the json file) a translation error will be reported.  

## Pre-requisites for usage
- git
- rust environment

## How to build the library
- checkout the source from the remote repository
- go to the root folder of the checked out sources
- for building a debug version type in cmd: cargo build
- for building a release version type in cmd: cargo build --release
  
## Usage 
In order to start translating the URLs two items are needed:  
- the full URL for translating
- a json file defining the translation rules (a example recipe.json file can be found in the examples)
  
## Running example code
There are two examples of how one can use the limiter from their rust application:  
- preloaded_args does not require user input. It showcases the translation on predefined recipes and link.  
```sh
cargo run --release --example preloaded_args
```
  
- load_args does the translation with user defined arguments  
```sh
cargo run --release --example load_args -- "<link>" "<path to recipes.json>"
```
  
