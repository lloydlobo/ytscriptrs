#!/bin/bash

# Bash script example to split a single large line into multiple lines ending with a period (.)

# The script prompts the user to input the single large line and then uses the 'sed' 
# command to replace all occurrences of '.' with '.,' followed by a newline character '\n'. 
# The resulting formatted line is then printed to the console.

# Note that this script assumes that the input line contains no other instances of 
# the '.' character that do not end a sentence. If the input line contains abbreviations 
# or other uses of the '.' character, this script may not produce the desired output.

# To pipe in the contents of a file, such as 'data.txt', into this Bash script, 
# you can use the '<' operator followed by the file name. For example:
#
# ```bash
# ./split_line.sh < data.txt
# ```
 
# This will execute the 'split_line.sh' script and use the contents of 'data.txt' as the input.
# This will add the execute permission (+x) to the file named script_name.sh. With this permission, you will be able to run the script.
#
# ```bash
# chmod +x script_name.sh
# ```

# Input the single large line
echo "Enter the single large line:"
read -r input_line

# Replace all occurrences of "." with ".\n" and print the result
formatted_line=$(echo "$input_line" | sed 's/\./.\n/g')
echo "Formatted line:"
echo "$formatted_line"

# It's not recommended to replace the contents of a file directly using the output of a script. If you want to write the output of the split_line.sh script to a file, you can use the output redirection operator > as follows:
#
# bash
#
# ./split_line.sh < data.txt > output.txt
#
# This will execute the split_line.sh script with the contents of data.txt as input, and write the output to a new file called output.txt. Note that if output.txt already exists, it will be overwritten. If you want to append the output to an existing file, you can use the >> operator instead of >. For example:
#
# bash
#
# ./split_line.sh < data.txt >> existing_output.txt
#
# This will execute the split_line.sh script with the contents of data.txt as input, and append the output to an existing file called existing_output.txt.
