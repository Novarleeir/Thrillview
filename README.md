# Thrillview
GUI for viewing and extracting assets from the Thrillville and Thrillville: Off The Rails OVL and ZAP game archives

<!-- TOC -->
* [Thrillview](#thrillview)
* [Usage:](#usage)
* [Development:](#development)
    * [Branches](#branches)
    * [Pull requests:](#pull-requests)
    * [Code formatting](#code-formatting)
<!-- TOC -->

# Usage:
___

todo

# Development:
___

### Branches

|  branch | usage                                      |
|--------:|--------------------------------------------|
|    main | for full stable releases                   |
| staging | stable-ish versions for release candidates |
|     dev | for breaking and/or new changes            |

___

### Pull requests:
Don't be scared to contribute, especially on the dev branch. I barely know what I'm doing myself, so I won't yell at you for trying or suggesting something. I'm using this project to learn rust, so if you have better ways to implement things please do share. However, when submitting a pull request, I do require that it's well documented/commented.
___
### Code formatting
I don't particularly care too much about having strict formatting guidelines, so long as it's readable. The only exception is for curly brackets/braces as shown below.

#### Enforced:
- Keeping starting brackets '{' on the same line as the start of the code block with no space.
    - ```
        fn thing(){
            //✔️
        }

        fn thing()
        {
            //❌
        }
      
        fn thing() {
            //❌
        }
      ```
#### Recommended, but optional:
- Using camelCase over rust's recommended snake_case, too bad!
  - Always having a return statement visible when returning a variable
    - `fn thing() -> bool{ return true; }`
    - instead of: `fn thing() -> bool{ true }`
  - Importing direct function namespaces
    - Ex: instead of `use std::*` or `use std::io::*` do: `use std::io::Error`
  - If an if/fn/loop/etc. only has one statement inside, feel free to keep it all inline
