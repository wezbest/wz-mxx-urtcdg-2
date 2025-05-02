#!/usr/bin/bash
# This bash srcript is for installing the KL docker image here
clear

# Colors
export RED='\033[0;31m'
export GREEN='\033[0;32m'
export YELLOW='\033[0;33m'
export BLUE='\033[0;34m'
export PURPLE='\033[0;35m'
export CYAN='\033[0;36m'
export WHITE='\033[0;37m'
export NC='\033[0m' # No Color

# Commands

hea1() {
    echo -e "${CYAN}~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~${NC}"
    echo -e "${PURPLE}$1${NC}"
    echo -e "${CYAN}~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~${NC}"
}

b1() {
    hea1 "UV Jupyter Lan Installation Commands"
}

# Function will ask for an input and convert to hex
encode_to_hex_input() {
    hea1 "Use cast to convert UTF8 to hex"
    echo -e ""
    echo -e "${BLUE}Write Text to encode: "
    echo -e "--------------------------------${NC}"
    read -r dataz
    if [ -z "$dataz" ]; then
        echo -e "${RED}BASTARD! PutSomething!${NC}"
        exit 1
    fi

    cmd1="cast fa \"$dataz\""
    hex_out=$(eval "$cmd1")
    cmd2="cast tas $hex_out"
    hex_in=$(eval "$cmd2")
    file_name="out.txt"

    echo -e "${CYAN}---Output---${NC}"
    echo -e "${GREEN} ${hex_out} ${NC}"
    echo "---Output---" >${file_name}
    echo "$hex_out" >>${file_name}
    echo -e ""
    echo -e "${YELLOW}---Input---${NC}"
    echo -e "${GREEN} ${hex_in} ${NC}"
    echo "---input---" >>${file_name}
    echo "$hex_in" >>${file_name}
}

#Function that will convert data stored in variable to hex
encodedata__to_hex_input() {
    hea1 "Use cast to convert UTF8 Data to hex"

    read -r -d '' dataz <<'EOF'

 ███████╗ ███╗   ███╗ ███████╗ ██╗      ██╗      ██████╗   █████╗  ███╗   ██╗ ████████╗ ██╗   ██╗
 ██╔════╝ ████╗ ████║ ██╔════╝ ██║      ██║      ██╔══██╗ ██╔══██╗ ████╗  ██║ ╚══██╔══╝ ╚██╗ ██╔╝
 ███████╗ ██╔████╔██║ █████╗   ██║      ██║      ██████╔╝ ███████║ ██╔██╗ ██║    ██║     ╚████╔╝ 
 ╚════██║ ██║╚██╔╝██║ ██╔══╝   ██║      ██║      ██╔═══╝  ██╔══██║ ██║╚██╗██║    ██║      ╚██╔╝  
 ███████║ ██║ ╚═╝ ██║ ███████╗ ███████╗ ███████╗ ██║      ██║  ██║ ██║ ╚████║    ██║       ██║   
 ╚══════╝ ╚═╝     ╚═╝ ╚══════╝ ╚══════╝ ╚══════╝ ╚═╝      ╚═╝  ╚═╝ ╚═╝  ╚═══╝    ╚═╝       ╚═╝   

EOF

    cmd1="cast fa \"$dataz\""
    hex_out=$(eval "$cmd1")
    file_name="out.txt"

    echo -e "${CYAN}---Output---${NC}"
    echo -e "${GREEN} ${hex_out} ${NC}"
    echo "---Output---" >${file_name}
    echo "$hex_out" >>${file_name}
    echo -e ""
}

# Send data with the transaction
ca_send_hex() {
    hea1 "Use cast to send hex data to multiple chains"

    # ASCII art data
    read -r -d '' dataz <<'EOF'

 ███████╗ ███╗   ███╗ ███████╗ ██╗      ██╗      ██████╗   █████╗  ███╗   ██╗ ████████╗ ██╗   ██╗
 ██╔════╝ ████╗ ████║ ██╔════╝ ██║      ██║      ██╔══██╗ ██╔══██╗ ████╗  ██║ ╚══██╔══╝ ╚██╗ ██╔╝
 ███████╗ ██╔████╔██║ █████╗   ██║      ██║      ██████╔╝ ███████║ ██╔██╗ ██║    ██║     ╚████╔╝ 
 ╚════██║ ██║╚██╔╝██║ ██╔══╝   ██║      ██║      ██╔═══╝  ██╔══██║ ██║╚██╗██║    ██║      ╚██╔╝  
 ███████║ ██║ ╚═╝ ██║ ███████╗ ███████╗ ███████╗ ██║      ██║  ██║ ██║ ╚████║    ██║       ██║   
 ╚══════╝ ╚═╝     ╚═╝ ╚══════╝ ╚══════╝ ╚══════╝ ╚═╝      ╚═╝  ╚═╝ ╚═╝  ╚═══╝    ╚═╝       ╚═╝   

EOF

    # Convert data to hex
    hex_out=$(cast fa "$dataz")

    # Wallet and key configurations
    local -a wallets=(
        "0x991A0FF9529bbC4E1b66cdb47e44DEeD1FcEE999" # Sender wallet
        "0x99F23c70837aa99175939077D34F20896CE8D399" # Recipient 1
        "0x995D96C5f70087cd6eA3c4F5eB8Ab7DeC3fDbe99" # Recipient 2
    )

    local -a keyz=(
        "0x15e64abfed3218cfe2ea1117e38eedb0a51990544534700e61cd803674be31ff" # Sender private key
        "0xe1eae1464d5fe82c12606b62ccdbe0eccb90e2d2134417b459dfb9dfda09f684" # Alternate key 1
        "0x17c674a1c7e43761479d09d76864c49d516e217006d965ae9df1fbf02ccc241d" # Alternate key 2
    )

    # Chain configurations (name:rpc_url)
    local -A chains=(
        ["sepolia"]="https://eth-sepolia.g.alchemy.com/v2/YfG5-esHajH3FpsLvC4eMFMEFYl9Lqcg"
        ["holesky"]="https://eth-holesky.g.alchemy.com/v2/YfG5-esHajH3FpsLvC4eMFMEFYl9Lqcg"
    )

    # Create log file with current date
    local log_file="send_hex_$(date +%Y-%m-%d_%H-%M-%S).log"
    echo "Transaction Log - $(date)" >"$log_file"
    echo "----------------------------------------" >>"$log_file"

    # Send to each chain
    local success_count=0
    local fail_count=0

    for chain in "${!chains[@]}"; do
        echo -e "\n${BLUE}Processing $chain chain...${NC}" | tee -a "$log_file"

        local command="cast send \
            --chain $chain \
            --rpc-url ${chains[$chain]} \
            --private-key ${keyz[0]} \
            ${wallets[1]} ${hex_out}"

        echo "[$(date +%T)] Executing: $command" >>"$log_file"

        local send_output
        send_output=$(eval "$command" 2>&1)
        local exit_code=$?

        if [ $exit_code -ne 0 ]; then
            echo -e "${RED}Error on $chain: ${send_output}${NC}" | tee -a "$log_file"
            ((fail_count++))
        else
            local tx_hash=$(echo "$send_output" | grep transactionHash | awk '{print $2}')
            echo -e "${GREEN}$chain successful! ${NC}" | tee -a "$log_file"
            echo -e "Transaction hash: ${CYAN}$tx_hash${NC}" | tee -a "$log_file"
            ((success_count++))
        fi

        echo "----------------------------------------" >>"$log_file"
        sleep 1 # Brief pause between chain transactions
    done

    # Summary
    echo -e "\n${WHITE}Transaction Summary:${NC}" | tee -a "$log_file"
    echo -e "${GREEN}Successful: $success_count${NC}" | tee -a "$log_file"
    echo -e "${RED}Failed: $fail_count${NC}" | tee -a "$log_file"
    echo -e "Detailed log: ${YELLOW}$log_file${NC}"

    # Return status
    if [ $fail_count -gt 0 ]; then
        return 1
    else
        return 0
    fi
}

# Send Reading File Data
ca_send_hex_file() {
    hea1 "Use cast to send hex data to multiple chains"

    # Configuration
    local ART_FILE="artz.txt" # Path to your ASCII art file

    # Verify art file exists and is not empty
    if [[ ! -f "$ART_FILE" ]]; then
        echo -e "${RED}Error: Art file '$ART_FILE' not found${NC}"
        return 1
    fi

    if [[ ! -s "$ART_FILE" ]]; then
        echo -e "${RED}Error: Art file '$ART_FILE' is empty${NC}"
        return 1
    fi

    # Read ASCII art from external file
    dataz=$(<"$ART_FILE")

    # Convert data to hex
    hex_out=$(cast fa "$dataz")

    # Wallet and key configurations
    local -a wallets=(
        "0x991A0FF9529bbC4E1b66cdb47e44DEeD1FcEE999" # Sender wallet
        "0x99F23c70837aa99175939077D34F20896CE8D399" # Recipient 1
        "0x995D96C5f70087cd6eA3c4F5eB8Ab7DeC3fDbe99" # Recipient 2
    )

    local -a keyz=(
        "0x15e64abfed3218cfe2ea1117e38eedb0a51990544534700e61cd803674be31ff" # Sender private key
        "0xe1eae1464d5fe82c12606b62ccdbe0eccb90e2d2134417b459dfb9dfda09f684" # Alternate key 1
        "0x17c674a1c7e43761479d09d76864c49d516e217006d965ae9df1fbf02ccc241d" # Alternate key 2
    )

    # Chain configurations (name:rpc_url)
    local -A chains=(
        ["sepolia"]="https://eth-sepolia.g.alchemy.com/v2/YfG5-esHajH3FpsLvC4eMFMEFYl9Lqcg"
        ["holesky"]="https://eth-holesky.g.alchemy.com/v2/YfG5-esHajH3FpsLvC4eMFMEFYl9Lqcg"
    )

    # Create log file with current date
    local log_file="send_hex_$(date +%Y-%m-%d_%H-%M-%S).log"
    echo "Transaction Log - $(date)" >"$log_file"
    echo "----------------------------------------" >>"$log_file"
    echo "ASCII Art Source: ${ART_FILE}" >>"$log_file"
    echo "----------------------------------------" >>"$log_file"

    # Send to each chain
    local success_count=0
    local fail_count=0

    for chain in "${!chains[@]}"; do
        echo -e "\n${BLUE}Processing $chain chain...${NC}" | tee -a "$log_file"

        local command="cast send \
            --chain $chain \
            --rpc-url ${chains[$chain]} \
            --private-key ${keyz[0]} \
            ${wallets[1]} ${hex_out}"

        echo "[$(date +%T)] Executing: $command" >>"$log_file"

        local send_output
        send_output=$(eval "$command" 2>&1)
        local exit_code=$?

        if [ $exit_code -ne 0 ]; then
            echo -e "${RED}Error on $chain: ${send_output}${NC}" | tee -a "$log_file"
            ((fail_count++))
        else
            local tx_hash=$(echo "$send_output" | grep transactionHash | awk '{print $2}')
            echo -e "${GREEN}$chain successful! ${NC}" | tee -a "$log_file"
            echo -e "Transaction hash: ${CYAN}$tx_hash${NC}" | tee -a "$log_file"
            ((success_count++))
        fi

        echo "----------------------------------------" >>"$log_file"
        sleep 1 # Brief pause between chain transactions
    done

    # Summary
    echo -e "\n${WHITE}Transaction Summary:${NC}" | tee -a "$log_file"
    echo -e "${GREEN}Successful: $success_count${NC}" | tee -a "$log_file"
    echo -e "${RED}Failed: $fail_count${NC}" | tee -a "$log_file"
    echo -e "Detailed log: ${YELLOW}$log_file${NC}"

    # Return status
    if [ $fail_count -gt 0 ]; then
        return 1
    else
        return 0
    fi
}

# Execution
ca_send_hex_file
