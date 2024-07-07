// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./PowerManagement.sol";
import "./FeeManagement.sol";

contract TaskManagement {

    FeeManagement public feeManagement;
    PowerManagement public powerManagement;
    event TasksAssigned(address indexed requester, string requestCID1, address worker1, address worker2, string requestCID2, address worker3, address worker4);
    event NextRoundStarted(string indexed selectedCID, address worker1, address worker2, address worker3, address worker4);
    event DataNeeds(string needs);

    constructor(address _feeManagementAddress, address _powerManagementAddress) {
        feeManagement = FeeManagement(_feeManagementAddress);
        powerManagement = PowerManagement(_powerManagementAddress);
    }

    function Initiatetraining(string memory _requestCID1, string memory _requestCID2) public {  
        //The training data of two data owners is assigned to four workers
        address[] memory registeredWorkers = powerManagement.getRegisteredWorkers();

        address worker1 = registeredWorkers[0];
        address worker2 = registeredWorkers[1];
        address worker3 = registeredWorkers[2];
        address worker4 = registeredWorkers[3];
    
        emit TasksAssigned(
            msg.sender,
            _requestCID1,
            worker1,
            worker2,
            _requestCID2,
            worker3,
            worker4
        );
    }

    function nextround(string memory selectedHash, string memory selectedCID) public {
        address[] memory registeredWorkers = powerManagement.getRegisteredWorkers();

        address worker1 = registeredWorkers[0];
        address worker2 = registeredWorkers[1];
        address worker3 = registeredWorkers[2];
        address worker4 = registeredWorkers[3];

        emit NextRoundStarted(
            selectedCID,
            worker1,
            worker2,
            worker3,
            worker4
        );
    }

    function dataneeds(string memory needs) public payable {
        // Call payFees function from FeeManagement contract
        feeManagement.payFees{value: msg.value}();
        emit DataNeeds(needs);
    }

}
