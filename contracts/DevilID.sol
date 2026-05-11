// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/**
 * @title DevilID
 * @notice Decentralized Identity system for DevilChain
 * @dev ERC-735 inspired self-sovereign identity with DAO verification
 */
contract DevilID {
    struct Identity {
        address owner;
        string username;         // Unique @handle
        string displayName;
        string avatarCID;        // IPFS/DevilStorage CID
        string bio;
        uint256 createdAt;
        uint256 reputationScore;
        bool daoVerified;        // DAO-verified identity
        bool active;
    }

    struct Credential {
        bytes32 id;
        address issuer;
        address subject;
        string credentialType;   // e.g. "email", "kyc", "developer"
        string dataHash;         // hash of credential data
        uint256 issuedAt;
        uint256 expiresAt;
        bool revoked;
    }

    mapping(address => Identity) public identities;
    mapping(string => address) public usernameToAddress;
    mapping(bytes32 => Credential) public credentials;
    mapping(address => bytes32[]) public userCredentials;

    address public daoContract;
    uint256 public totalIdentities;

    event IdentityCreated(address indexed owner, string username);
    event IdentityUpdated(address indexed owner);
    event CredentialIssued(bytes32 indexed credId, address indexed subject);
    event CredentialRevoked(bytes32 indexed credId);
    event DAOVerified(address indexed owner);

    modifier onlyDAO() {
        require(msg.sender == daoContract, "Only DAO");
        _;
    }

    modifier hasIdentity() {
        require(identities[msg.sender].active, "No identity found");
        _;
    }

    constructor(address _daoContract) {
        daoContract = _daoContract;
    }

    function createIdentity(
        string calldata username,
        string calldata displayName,
        string calldata bio,
        string calldata avatarCID
    ) external {
        require(!identities[msg.sender].active, "Identity exists");
        require(bytes(username).length >= 3, "Username too short");
        require(usernameToAddress[username] == address(0), "Username taken");

        identities[msg.sender] = Identity({
            owner: msg.sender,
            username: username,
            displayName: displayName,
            avatarCID: avatarCID,
            bio: bio,
            createdAt: block.timestamp,
            reputationScore: 0,
            daoVerified: false,
            active: true
        });

        usernameToAddress[username] = msg.sender;
        totalIdentities++;

        emit IdentityCreated(msg.sender, username);
    }

    function updateIdentity(
        string calldata displayName,
        string calldata bio,
        string calldata avatarCID
    ) external hasIdentity {
        Identity storage id = identities[msg.sender];
        id.displayName = displayName;
        id.bio = bio;
        id.avatarCID = avatarCID;
        emit IdentityUpdated(msg.sender);
    }

    function issueCredential(
        address subject,
        string calldata credType,
        string calldata dataHash,
        uint256 expiresAt
    ) external returns (bytes32 credId) {
        credId = keccak256(abi.encodePacked(msg.sender, subject, credType, block.timestamp));
        credentials[credId] = Credential({
            id: credId,
            issuer: msg.sender,
            subject: subject,
            credentialType: credType,
            dataHash: dataHash,
            issuedAt: block.timestamp,
            expiresAt: expiresAt,
            revoked: false
        });
        userCredentials[subject].push(credId);
        emit CredentialIssued(credId, subject);
    }

    function revokeCredential(bytes32 credId) external {
        require(credentials[credId].issuer == msg.sender, "Not issuer");
        credentials[credId].revoked = true;
        emit CredentialRevoked(credId);
    }

    function daoVerify(address user) external onlyDAO {
        require(identities[user].active, "No identity");
        identities[user].daoVerified = true;
        identities[user].reputationScore += 100;
        emit DAOVerified(user);
    }

    function resolveUsername(string calldata username) external view returns (address) {
        return usernameToAddress[username];
    }

    function getIdentity(address user) external view returns (Identity memory) {
        return identities[user];
    }

    function getUserCredentials(address user) external view returns (bytes32[] memory) {
        return userCredentials[user];
    }

    function isCredentialValid(bytes32 credId) external view returns (bool) {
        Credential memory c = credentials[credId];
        return !c.revoked && (c.expiresAt == 0 || block.timestamp < c.expiresAt);
    }
}
