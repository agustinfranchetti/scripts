# Scripts

###### This repository contains a collection of Rust-based scripts for personal use. The scripts are designed to automate various tasks.

## Getting Started

###### To use the scripts in this repository, follow the instructions below.

### Prerequisites

- Rust: Make sure you have Rust installed on your machine. You can install Rust by following the official installation instructions: https://www.rust-lang.org/tools/install

### Cloning the Repository

1. Open a terminal.

2. Clone the repository using the following command:

   ```bash
   git clone https://github.com/agustinfranchetti/scripts.git```
3. Navigate to the cloned repository:

    ```bash 
    cd scripts
### Using the Scripts
1. Navigate to the directory containing the script you want to use.

2. Compile the script using the following command:

    ```bash 
    rustc script-name.rs
    ```
    //Replace script-name.rs with the actual name of the Rust script file.

3. After successful compilation, you can execute the script using the following command:

    ```bash
    ./script-name
    ```
    //Replace script-name with the name of the compiled script file.

### Assigning Aliases (Optional)
###### If you want to assign aliases to the scripts for easier execution, follow the steps below:

1. Open your shell configuration file using a text editor. For example, if you're using Bash, you can use the following command to open the .bashrc file:

    ```bash 
    nano ~/.bashrc
    ```
    If you`re using Zsh, open the .zshrc file instead:
    ```bash
    nano ~/.zshrc
    ```
2. Add the following line at the end of the file for each script you want to alias:

    ```bash
    alias alias-name='/path/to/compiled/script'
    ```
    Replace alias-name with your desired alias and /path/to/compiled/script with the actual path to the compiled script file.

3. Save the changes and close the text editor.

4. To apply the changes, either restart your terminal or run the following command:

    ```bash
    source ~/.bashrc
    ```
    or
    ```bash
    source ~/.zshrc
    ```
5. Now you can simply use the assigned aliases to execute the scripts.

### Modifying and Recompiling
###### If you want to modify a script and recompile it, follow the steps below:

1. Open the script file using a text editor.

2. Make the necessary modifications to the script.

3. Save the changes.

4. In the terminal, navigate to the directory containing the modified script.

5. Recompile the script using the following command:
    ```bash
    rustc script-name.rs
    ```
    Replace script-name.rs with the actual name of the modified script file.

6. After successful compilation, you can execute the updated script using the instructions mentioned earlier.

### Contributing
Feel free to contribute to this repository by adding new scripts or improving the existing ones. Submit your changes as pull requests, and they will be reviewed.
