// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./FeeManagement.sol";
import "./PowerManagement.sol";

contract RegistrationContract {
    
    FeeManagement public feeManagement;
    PowerManagement public powerManagement;
    
    struct Requester {
        address requesterAddress;
        string name;
        string description;
        string zkProgramCID;
        string accessRequirements;
        string dataDescription;
        uint feePaid;
    }

    struct DataOwner {
        address dataOwnerAddress;
        string name;
        string description;
        string requestCID;
    }

    struct Validator {
        address validatorAddress;
        string name;
        string description;
        uint stakedToken;
    }
    
    mapping(address => Requester) public requesters;
    mapping(address => DataOwner) public dataOwners;
    mapping(address => Validator) public validators;
    
    event ValidatorRegistered(address indexed validatorAddress, string name, string description, uint stakedToken);
    event ValidatorUnregistered(address indexed validatorAddress, uint returnedToken);
    event RequesterRegistered(address indexed requesterAddress, string name, string description, string zkProgramCID, uint feePaid);
    event DataOwnerRegistered(address indexed dataOwnerAddress, string name, string description, string requestCID);
    event WorkerRegistered(address indexed workerAddress, string name, string description, address ownerAddress);
    
    constructor(address _feeManagementAddress, address _powerManagementAddress) {  //构造函数，开始时自动执行，需要修改
        feeManagement = FeeManagement(_feeManagementAddress);
        //0xd9145CCE52D386f254917e481eB44e9943F39138
        powerManagement = PowerManagement(_powerManagementAddress);
        //0xd8b934580fcE35a11B58C6D73aDeE468a2833fa8
    }
    
    // 注册Requester
    function registerRequester(
        string memory _name,
        string memory _description,
        string calldata _zkProgramCID, 
        string memory _accessRequirements, 
        string memory _dataDescription
    ) public payable {
        require(requesters[msg.sender].requesterAddress == address(0), "Requester already registered");
        
        // 调用FeeManagement合约进行支付
        feeManagement.payFees{value: msg.value}();
        
        requesters[msg.sender] = Requester({   
            requesterAddress: msg.sender,
            name: _name,
            description: _description,
            zkProgramCID: _zkProgramCID,
            accessRequirements: _accessRequirements,
            dataDescription: _dataDescription,
            feePaid: msg.value
        });
        
        emit RequesterRegistered(msg.sender, _name, _description, _zkProgramCID, msg.value);
    }
    
    // 注册Data Owner
    function registerDataOwner(
        string memory _name,
        string memory _description,
        string calldata _requestCID
    ) public {
        require(dataOwners[msg.sender].dataOwnerAddress == address(0), "Data Owner already registered");
        
        dataOwners[msg.sender] = DataOwner({   //相当于写入数组，  dataowner[地址]= 数据。
            dataOwnerAddress: msg.sender,
            name: _name,
            description: _description,
            requestCID: _requestCID
        });
        
        emit DataOwnerRegistered(msg.sender, _name, _description, _requestCID);
    }
    
    // 注册Worker
    function registerWorker(
        string memory _name,
        string memory _description,
        string memory _machineResources,
        string memory _rpcAddress,
        uint16 _port,
        string memory _publicKey
    ) public {
        // 调用Power Management合约进行worker注册
        powerManagement.registerWorker(_name, _description, _machineResources, _rpcAddress, _port, _publicKey);
        
        emit WorkerRegistered(msg.sender, _name, _description, msg.sender);
    }
    
    // 注册Validator
    function registerValidator(
        string memory _name, 
        string memory _description
    ) public payable {
        require(validators[msg.sender].validatorAddress == address(0), "Validator already registered");
        require(msg.value > 0, "Validator must stake tokens");
        
        // 调用FeeManagement合约进行质押
        feeManagement.stakeTokens{value: msg.value}();
        
        validators[msg.sender] = Validator({  //注册成功的，质押token的validators
            validatorAddress: msg.sender,
            name: _name,
            description: _description,
            stakedToken: msg.value
        });
        
        emit ValidatorRegistered(msg.sender, _name, _description, msg.value);
    }
    
    // 注销Validator
    function unregisterValidator() public {
        require(validators[msg.sender].validatorAddress != address(0), "Validator not registered");
        
        // 调用FeeManagement合约退还质押的Token
        feeManagement.unstakeTokens();
        
        // 清除Validator的注册信息
        delete validators[msg.sender];
        
        emit ValidatorUnregistered(msg.sender, validators[msg.sender].stakedToken);
    }
    
    // 查询某个Validator的信息
    function getValidatorInfo(address _validatorAddress) public view returns (Validator memory) {
        return validators[_validatorAddress];
    }
    
    // 查询某个Requester的信息
    function getRequesterInfo(address _requesterAddress) public view returns (Requester memory) {
        return requesters[_requesterAddress];
    }
    
    // 查询某个Data Owner的信息
    function getDataOwnerInfo(address _dataOwnerAddress) public view returns (DataOwner memory) {
        return dataOwners[_dataOwnerAddress];
    }
}
