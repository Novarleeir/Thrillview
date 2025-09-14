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
I don't particularly care too much about having strict formatting guidelines, so long as it's readable.

- I'll list a few of my conventions that I personally prefer. Frankly, I don't care how much rust complains about having a formatting standard, too bad! However, If you're submitting a PR, it's optional to follow these:
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
  - Using camelCase over rust's recommended snake_case, too bad!
  - Always having a return statement when returning a variable
    - `fn thing() -> bool{ return true; }`
    - instead of: `fn thing() -> bool{ true }`
  - Commenting:
    - I use a plugin for rust rover that highlights comments that have certain characters in them, which explains why some comments may have stuff like: (-, ~, *, !, Note:)
    - Doc comments: `/// Capitalized, with a space at the beginning`
    - Regular comments: `//uncapitalized, with no space`
  - Importing direct function namespaces
    - Ex: instead of `use std::*` or `use std::io::*` do: `use std::io::Error`
  - If an if/fn/loop/etc. only has one statement inside, feel free to keep it all inline
