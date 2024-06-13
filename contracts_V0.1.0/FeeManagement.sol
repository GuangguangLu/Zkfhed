// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract FeeManagement {

    address public owner;

    mapping(address => uint) public stakedBalances;  //质押token
    mapping(address => uint) public feesPaid;   //requester 支付 token
    uint public totalFees;

    event TokensStaked(address indexed staker, uint amount);
    event TokensUnstaked(address indexed staker, uint amount);
    event FeesPaid(address indexed payer, uint amount);
    event FeesDistributed(uint totalAmount, address indexed validator, address indexed worker, address indexed dataOwner);

    constructor() {
        owner = msg.sender;
    }

    // Validator质押Token
    function stakeTokens() public payable {
        require(msg.value > 0, "Must stake a positive amount of tokens");
        stakedBalances[msg.sender] += msg.value;
        emit TokensStaked(msg.sender, msg.value);
    }

    // Validator注销并退还Token
    function unstakeTokens() public {
        uint amount = stakedBalances[msg.sender];
        require(amount > 0, "No tokens to unstake");
        stakedBalances[msg.sender] = 0;
        payable(msg.sender).transfer(amount);
        emit TokensUnstaked(msg.sender, amount);
    }

    // Requester支付费用
    function payFees() public payable {
        require(msg.value > 0, "Must pay a positive amount of fees");
        feesPaid[msg.sender] += msg.value;
        totalFees += msg.value;
        emit FeesPaid(msg.sender, msg.value);
    }

    // 任务结束后分配费用，example
    function distributeFees(
        address payable[] memory validators, 
        address payable[] memory workers, 
        address payable[] memory dataOwners
    ) public {
        require(validators.length > 0, "Validators list cannot be empty");
        require(workers.length > 0, "Workers list cannot be empty");
        require(dataOwners.length > 0, "Data Owners list cannot be empty");

        uint validatorShare = totalFees * 30 / 100 / validators.length;
        uint workerShare = totalFees * 50 / 100 / workers.length;
        uint dataOwnerShare = totalFees * 20 / 100 / dataOwners.length;

        // Reset total fees before transferring to avoid re-entrancy attacks
        totalFees = 0;

        // 分配给validators
        for (uint i = 0; i < validators.length; i++) {
            validators[i].transfer(validatorShare);
        }

        // 分配给workers
        for (uint i = 0; i < workers.length; i++) {
            workers[i].transfer(workerShare);
        }

        // 分配给dataOwners
        for (uint i = 0; i < dataOwners.length; i++) {
            dataOwners[i].transfer(dataOwnerShare);
        }

        emit FeesDistributed(validatorShare * validators.length + workerShare * workers.length + dataOwnerShare * dataOwners.length, validators, workers, dataOwners);
    }

    // 获取质押的Token余额
    function getStakedBalance(address staker) public view returns (uint) {
        return stakedBalances[staker];
    }

    // 获取支付的费用余额
    function getFeesPaid(address payer) public view returns (uint) {
        return feesPaid[payer];
    }
}
