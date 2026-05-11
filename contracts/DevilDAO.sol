// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "./DevilStaking.sol";

/**
 * @title DevilChain DAO
 * @notice On-chain governance for DevilChain Network
 */
contract DevilDAO {
    DevilStaking public staking;

    struct Proposal {
        uint256 id;
        string title;
        string description;
        address proposer;
        uint256 votesFor;
        uint256 votesAgainst;
        bool executed;
        uint256 deadline;
    }

    uint256 public proposalCount;
    mapping(uint256 => Proposal) public proposals;
    mapping(uint256 => mapping(address => bool)) public hasVoted;

    event ProposalCreated(uint256 indexed id, address proposer, string title);
    event Voted(uint256 indexed id, address voter, bool inFavor, uint256 power);
    event ProposalExecuted(uint256 indexed id, bool passed);

    constructor(address stakingAddress) {
        staking = DevilStaking(stakingAddress);
    }

    function createProposal(string calldata title, string calldata description) external returns (uint256) {
        proposalCount++;
        proposals[proposalCount] = Proposal({
            id: proposalCount,
            title: title,
            description: description,
            proposer: msg.sender,
            votesFor: 0,
            votesAgainst: 0,
            executed: false,
            deadline: block.timestamp + 7 days
        });
        emit ProposalCreated(proposalCount, msg.sender, title);
        return proposalCount;
    }

    function vote(uint256 proposalId, bool inFavor) external {
        Proposal storage p = proposals[proposalId];
        require(block.timestamp < p.deadline, "Voting closed");
        require(!hasVoted[proposalId][msg.sender], "Already voted");
        uint256 power = staking.votingPower(msg.sender);
        require(power > 0, "No voting power");
        hasVoted[proposalId][msg.sender] = true;
        if (inFavor) { p.votesFor += power; } else { p.votesAgainst += power; }
        emit Voted(proposalId, msg.sender, inFavor, power);
    }

    function executeProposal(uint256 proposalId) external {
        Proposal storage p = proposals[proposalId];
        require(block.timestamp >= p.deadline, "Voting ongoing");
        require(!p.executed, "Already executed");
        p.executed = true;
        bool passed = p.votesFor > p.votesAgainst;
        emit ProposalExecuted(proposalId, passed);
    }
}
