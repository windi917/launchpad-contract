export type PresaleContract = {
  "version": "0.1.0",
  "name": "presale_contract",
  "instructions": [
    {
      "name": "initialize",
      "docs": [
        "* @dev Initialize the project"
      ],
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "createPresale",
      "accounts": [
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "presale",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "createrTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "destTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenMintAddress",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "minAllocation",
          "type": "u64"
        },
        {
          "name": "maxAllocation",
          "type": "u64"
        },
        {
          "name": "hardcap",
          "type": "u64"
        },
        {
          "name": "softcap",
          "type": "u64"
        },
        {
          "name": "salePrice",
          "type": "u64"
        },
        {
          "name": "launchPrice",
          "type": "u64"
        },
        {
          "name": "startTime",
          "type": "i64"
        },
        {
          "name": "endTime",
          "type": "i64"
        },
        {
          "name": "maxContribution",
          "type": "u64"
        },
        {
          "name": "baseDecimals",
          "type": "u64"
        },
        {
          "name": "baseMint",
          "type": "publicKey"
        },
        {
          "name": "quoteMint",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "buyTokens",
      "accounts": [
        {
          "name": "buyer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "presale",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "creator",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userBaseTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalBaseTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userQuoteTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalQuoteTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "baseDecimals",
          "type": "u64"
        },
        {
          "name": "isNative",
          "type": "u64"
        },
        {
          "name": "globalBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "withdraw",
      "accounts": [
        {
          "name": "buyer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "presale",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "creator",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userBaseTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalBaseTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userQuoteTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalQuoteTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "adminQuoteTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "baseDecimals",
          "type": "u64"
        },
        {
          "name": "isNative",
          "type": "u64"
        },
        {
          "name": "globalBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "setApprove",
      "accounts": [
        {
          "name": "presale",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "global",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "presale",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "minAllocation",
            "type": "u64"
          },
          {
            "name": "maxAllocation",
            "type": "u64"
          },
          {
            "name": "hardcap",
            "type": "u64"
          },
          {
            "name": "softcap",
            "type": "u64"
          },
          {
            "name": "salePrice",
            "type": "u64"
          },
          {
            "name": "launchPrice",
            "type": "u64"
          },
          {
            "name": "startTime",
            "type": "i64"
          },
          {
            "name": "endTime",
            "type": "i64"
          },
          {
            "name": "totalContributions",
            "type": "u64"
          },
          {
            "name": "maxContribution",
            "type": "u64"
          },
          {
            "name": "baseMint",
            "type": "publicKey"
          },
          {
            "name": "quoteMint",
            "type": "publicKey"
          },
          {
            "name": "state",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "PresaleStart",
      "msg": "The presale is not started."
    },
    {
      "code": 6001,
      "name": "PresaleEnd",
      "msg": "The presale is ended."
    },
    {
      "code": 6002,
      "name": "PresaleNotEnded",
      "msg": "The presale is not ended."
    },
    {
      "code": 6003,
      "name": "InvalidContributionAmount",
      "msg": "The contribution amount is invalid."
    },
    {
      "code": 6004,
      "name": "HardcapExceeded",
      "msg": "The hardcap has been exceeded."
    },
    {
      "code": 6005,
      "name": "NotApproved",
      "msg": "Proposal does not approved."
    },
    {
      "code": 6006,
      "name": "Withdrawed",
      "msg": "Already withdrawed."
    },
    {
      "code": 6007,
      "name": "NotAvaliable",
      "msg": "Presale is not avaliable now."
    }
  ]
};

export const IDL: PresaleContract = {
  "version": "0.1.0",
  "name": "presale_contract",
  "instructions": [
    {
      "name": "initialize",
      "docs": [
        "* @dev Initialize the project"
      ],
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "createPresale",
      "accounts": [
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "presale",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "createrTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "destTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenMintAddress",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "minAllocation",
          "type": "u64"
        },
        {
          "name": "maxAllocation",
          "type": "u64"
        },
        {
          "name": "hardcap",
          "type": "u64"
        },
        {
          "name": "softcap",
          "type": "u64"
        },
        {
          "name": "salePrice",
          "type": "u64"
        },
        {
          "name": "launchPrice",
          "type": "u64"
        },
        {
          "name": "startTime",
          "type": "i64"
        },
        {
          "name": "endTime",
          "type": "i64"
        },
        {
          "name": "maxContribution",
          "type": "u64"
        },
        {
          "name": "baseDecimals",
          "type": "u64"
        },
        {
          "name": "baseMint",
          "type": "publicKey"
        },
        {
          "name": "quoteMint",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "buyTokens",
      "accounts": [
        {
          "name": "buyer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "presale",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "creator",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userBaseTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalBaseTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userQuoteTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalQuoteTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "baseDecimals",
          "type": "u64"
        },
        {
          "name": "isNative",
          "type": "u64"
        },
        {
          "name": "globalBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "withdraw",
      "accounts": [
        {
          "name": "buyer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "presale",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "creator",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userBaseTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalBaseTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userQuoteTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalQuoteTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "adminQuoteTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "baseDecimals",
          "type": "u64"
        },
        {
          "name": "isNative",
          "type": "u64"
        },
        {
          "name": "globalBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "setApprove",
      "accounts": [
        {
          "name": "presale",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "global",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "presale",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "minAllocation",
            "type": "u64"
          },
          {
            "name": "maxAllocation",
            "type": "u64"
          },
          {
            "name": "hardcap",
            "type": "u64"
          },
          {
            "name": "softcap",
            "type": "u64"
          },
          {
            "name": "salePrice",
            "type": "u64"
          },
          {
            "name": "launchPrice",
            "type": "u64"
          },
          {
            "name": "startTime",
            "type": "i64"
          },
          {
            "name": "endTime",
            "type": "i64"
          },
          {
            "name": "totalContributions",
            "type": "u64"
          },
          {
            "name": "maxContribution",
            "type": "u64"
          },
          {
            "name": "baseMint",
            "type": "publicKey"
          },
          {
            "name": "quoteMint",
            "type": "publicKey"
          },
          {
            "name": "state",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "PresaleStart",
      "msg": "The presale is not started."
    },
    {
      "code": 6001,
      "name": "PresaleEnd",
      "msg": "The presale is ended."
    },
    {
      "code": 6002,
      "name": "PresaleNotEnded",
      "msg": "The presale is not ended."
    },
    {
      "code": 6003,
      "name": "InvalidContributionAmount",
      "msg": "The contribution amount is invalid."
    },
    {
      "code": 6004,
      "name": "HardcapExceeded",
      "msg": "The hardcap has been exceeded."
    },
    {
      "code": 6005,
      "name": "NotApproved",
      "msg": "Proposal does not approved."
    },
    {
      "code": 6006,
      "name": "Withdrawed",
      "msg": "Already withdrawed."
    },
    {
      "code": 6007,
      "name": "NotAvaliable",
      "msg": "Presale is not avaliable now."
    }
  ]
};
