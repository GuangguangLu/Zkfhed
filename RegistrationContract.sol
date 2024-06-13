// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./FeeManagement.sol";
import "./PowerManagement.sol";

contract RegistrationContract {
    
    FeeManagement public feeManagement;
    PowerManagement public powerManagement;

    event RequesterRegistered(address indexed requesterAddress, string zkProgramCID, string accessRequirements, string dataDescription, uint feePaid);
    event DataOwnerRegistered(address indexed dataOwnerAddress, string requestCID);
    event WorkerRegistered(address indexed workerAddress, string machineResources);
    event ValidatorRegistered(address indexed validatorAddress, string machineResources, uint stakedToken);

    constructor(address _feeManagementAddress, address _powerManagementAddress) {
        feeManagement = FeeManagement(_feeManagementAddress);
        powerManagement = PowerManagement(_powerManagementAddress);
    }

    // 注册Requester
    function registerRequester(
        string calldata _zkProgramCID,
        string calldata _accessRequirements,
        string calldata _dataDescription
    ) public payable {
        // 调用FeeManagement合约进行支付
        feeManagement.payFees{value: msg.value}();

        emit RequesterRegistered(
            msg.sender,
            _zkProgramCID,
            _accessRequirements,
            _dataDescription,
            msg.value
        );
    }
    
    // 注册Data Owner
    function registerDataOwner(
        string calldata _requestCID
    ) public {
        emit DataOwnerRegistered(msg.sender, _requestCID);
    }
    
    // 注册Worker
    function registerWorker(
        string calldata _machineResources
    ) public {
        // 调用Power Management合约进行worker注册
        powerManagement.registerWorker(_machineResources);

        emit WorkerRegistered(msg.sender, _machineResources);
    }
    
// 注册Validator
    function registerValidator(
        string calldata machineResources
    ) public payable {

        // 调用FeeManagement合约进行质押
        feeManagement.stakeTokens{value: msg.value}();

        emit ValidatorRegistered(msg.sender, machineResources, msg.value);
    }
}
