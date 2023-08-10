# Link Transfomer
## Small rust library package for transforming URLs

This library can come hand when:
1) You work with internet web adresses while being outside of the private intranet network.
  Those resources may still be accessible but the base URL has to be changed to publicly exposed one.

2) In the intranet you work with network drives which are not reachable from outside of the network.
   Often there is alternative web url that lead to this recource. This lib will do translation for you.
   As a bonus in case hidden folders are used a dollar sign will be added to the chosen subfolders.

3) The webservice that you like has a shorthand URL which is now no longer supported.
   Thanks to this solution you won't need to memorizy the full often dificult to remeber web-domain adress
  
# How does it work 
    The library simply replaces the base URL of the provided link with the matching URL found in a recipes.json file.
    If the base URL is unkown (not mentioned in the json file) a translation error will be reported.

# Pre-requisites for usage
    - git 
    - rust enviorment

# How to build the library
    - checkout the source from gitab
    - go to the root folder of the checkedout sources
    - for building a debug version type in cmd: cargo build --release
    - for building a relese version type in cmd: cargo build --release
  
# Usage 
 In order to start translating the URLs two items are needed:
    - the full URL for translating
    - a json file defining the translation rules (a example recipe.json file can be found in the exmaples)
    
# Runnig example code
 There are two examples of how one can use the limiter from their rust application:
- preloaded_args does not require user input. It showcases the transaltion on predifined recipes and link.
    ```sh
    cargo run --release --example preloaded_args
    ```
- load_args does the transaltion with user defined arguments
    ```sh
    cargo run --release --example load_args -- "<link>" "<path to recipes.json>"
    ```
 