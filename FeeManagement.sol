// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract FeeManagement {

    mapping(address => uint) public stakedBalances;  
    mapping(address => uint) public feesPaid;   
    uint public totalFees;

    function stakeTokens() public payable {
        require(msg.value > 0);
        stakedBalances[msg.sender] += msg.value;
    }

    function unstakeTokens() public {
        uint amount = stakedBalances[msg.sender];
        require(amount > 0);
        stakedBalances[msg.sender] = 0;
        payable(msg.sender).transfer(amount);
    }


    function payFees() public payable {
        feesPaid[msg.sender] += msg.value;
        totalFees += msg.value;
    }


    function distributeFees(address[] memory entities) public {

        uint sharePerEntity = totalFees / entities.length;

        // Reset total fees before transferring to avoid re-entrancy attacks
        totalFees = 0;

        for (uint i = 0; i < entities.length; i++) {
            require(entities[i] != address(0));
            payable(entities[i]).transfer(sharePerEntity);
        }
    }

}