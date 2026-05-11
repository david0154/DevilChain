// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "./DevilCoin.sol";

/**
 * @title DevilStaking
 * @notice Proof-of-Stake staking contract for DevilChain validators
 */
contract DevilStaking {
    DevilCoin public dvc;
    uint256 public minStake = 100 * 10**18; // 100 DVC minimum

    struct Validator {
        address addr;
        uint256 staked;
        uint256 reputationScore;
        bool active;
    }

    mapping(address => Validator) public validators;
    address[] public validatorList;

    event Staked(address indexed validator, uint256 amount);
    event Unstaked(address indexed validator, uint256 amount);
    event ValidatorActivated(address indexed validator);

    constructor(address dvcAddress) {
        dvc = DevilCoin(dvcAddress);
    }

    function stake(uint256 amount) external {
        require(amount >= minStake, "Below minimum stake");
        dvc.transferFrom(msg.sender, address(this), amount);
        if (!validators[msg.sender].active) {
            validators[msg.sender] = Validator(msg.sender, amount, 0, true);
            validatorList.push(msg.sender);
            emit ValidatorActivated(msg.sender);
        } else {
            validators[msg.sender].staked += amount;
        }
        emit Staked(msg.sender, amount);
    }

    function unstake(uint256 amount) external {
        Validator storage v = validators[msg.sender];
        require(v.staked >= amount, "Insufficient stake");
        v.staked -= amount;
        if (v.staked < minStake) {
            v.active = false;
        }
        dvc.transfer(msg.sender, amount);
        emit Unstaked(msg.sender, amount);
    }

    function votingPower(address validator) external view returns (uint256) {
        Validator memory v = validators[validator];
        // Voting Power = Stake + Reputation
        return v.staked + v.reputationScore;
    }

    function getValidators() external view returns (address[] memory) {
        return validatorList;
    }
}
