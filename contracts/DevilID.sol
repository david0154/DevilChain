// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title DevilID — Decentralized Identity System on DevilChain
/// @notice Create, manage and verify on-chain identities
contract DevilID {
    struct Identity {
        string  username;       // Unique username e.g. @devil
        string  displayName;    // Display name
        string  bio;            // Profile bio
        string  avatarCid;      // IPFS CID for avatar
        address wallet;         // Linked wallet address
        uint256 createdAt;      // Creation timestamp
        uint256 reputationScore; // Reputation score (DAO-adjustable)
        bool    isVerified;     // DAO-verified identity
        bool    isActive;       // Active status
    }

    mapping(address => Identity)  private identities;    // wallet => Identity
    mapping(string => address)    private usernameToAddr; // username => wallet
    mapping(address => address[]) private following;      // follower => [following...]
    mapping(address => uint256)   private followerCount;  // address => follower count

    uint256 public totalIdentities;
    address public dao;  // DAO contract for verification

    event IdentityCreated(address indexed wallet, string username);
    event IdentityUpdated(address indexed wallet, string username);
    event IdentityVerified(address indexed wallet, bool status);
    event Followed(address indexed follower, address indexed target);
    event Unfollowed(address indexed follower, address indexed target);

    modifier onlyIdentityOwner() {
        require(identities[msg.sender].wallet == msg.sender, "DevilID: No identity found");
        _;
    }

    modifier onlyDAO() {
        require(msg.sender == dao, "DevilID: Only DAO");
        _;
    }

    constructor(address _dao) {
        dao = _dao;
    }

    /// @notice Create a new DevilID
    function createIdentity(
        string calldata username,
        string calldata displayName,
        string calldata bio,
        string calldata avatarCid
    ) external {
        require(identities[msg.sender].wallet == address(0), "Identity already exists");
        require(usernameToAddr[username] == address(0), "Username taken");
        require(bytes(username).length >= 3 && bytes(username).length <= 32, "Username 3-32 chars");

        identities[msg.sender] = Identity({
            username: username,
            displayName: displayName,
            bio: bio,
            avatarCid: avatarCid,
            wallet: msg.sender,
            createdAt: block.timestamp,
            reputationScore: 100,
            isVerified: false,
            isActive: true
        });

        usernameToAddr[username] = msg.sender;
        totalIdentities++;
        emit IdentityCreated(msg.sender, username);
    }

    /// @notice Update identity profile
    function updateIdentity(
        string calldata displayName,
        string calldata bio,
        string calldata avatarCid
    ) external onlyIdentityOwner {
        Identity storage id = identities[msg.sender];
        id.displayName = displayName;
        id.bio = bio;
        id.avatarCid = avatarCid;
        emit IdentityUpdated(msg.sender, id.username);
    }

    /// @notice Get identity by wallet address
    function getIdentity(address wallet) external view returns (Identity memory) {
        require(identities[wallet].isActive, "Identity not found or inactive");
        return identities[wallet];
    }

    /// @notice Look up address by username
    function resolveUsername(string calldata username) external view returns (address) {
        address addr = usernameToAddr[username];
        require(addr != address(0), "Username not found");
        return addr;
    }

    /// @notice DAO can verify identities
    function setVerified(address wallet, bool status) external onlyDAO {
        identities[wallet].isVerified = status;
        emit IdentityVerified(wallet, status);
    }

    /// @notice DAO can adjust reputation scores
    function adjustReputation(address wallet, uint256 score) external onlyDAO {
        identities[wallet].reputationScore = score;
    }

    /// @notice Follow another DevilID
    function follow(address target) external onlyIdentityOwner {
        require(target != msg.sender, "Cannot follow yourself");
        following[msg.sender].push(target);
        followerCount[target]++;
        emit Followed(msg.sender, target);
    }

    /// @notice Deactivate own identity
    function deactivate() external onlyIdentityOwner {
        identities[msg.sender].isActive = false;
    }

    /// @notice Get who an address follows
    function getFollowing(address wallet) external view returns (address[] memory) {
        return following[wallet];
    }

    /// @notice Get follower count
    function getFollowerCount(address wallet) external view returns (uint256) {
        return followerCount[wallet];
    }

    /// @notice Check if address has an active identity
    function hasIdentity(address wallet) external view returns (bool) {
        return identities[wallet].wallet != address(0) && identities[wallet].isActive;
    }
}
