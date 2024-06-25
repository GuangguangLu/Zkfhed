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

    // registerRequester
    function registerRequester(
        string calldata _zkProgramCID,
        string calldata _accessRequirements,
        string calldata _dataDescription
    ) public payable {
        //Calling the FeeManagement contract for payment
        feeManagement.payFees{value: msg.value}();

        emit RequesterRegistered(
            msg.sender,
            _zkProgramCID,
            _accessRequirements,
            _dataDescription,
            msg.value
        );
    }
    
    // registerDataOwner
    function registerDataOwner(
        string calldata _requestCID
    ) public {
        emit DataOwnerRegistered(msg.sender, _requestCID);
    }
    
    // registerWorker
    function registerWorker(
        string calldata _machineResources
    ) public {
        // Calling the Power Management contract for worker registration
        powerManagement.registerWorker(_machineResources);

        emit WorkerRegistered(msg.sender, _machineResources);
    }
    
// registerValidator
    function registerValidator(
        string calldata machineResources
    ) public payable {

        // Call FeeManagement contract for staking
        feeManagement.stakeTokens{value: msg.value}();

        emit ValidatorRegistered(msg.sender, machineResources, msg.value);
    }
}
