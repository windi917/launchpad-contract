export type Solucky = {
  "version": "0.1.0",
  "name": "solucky",
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
      "name": "createRaffle",
      "docs": [
        "* @dev Create new raffle with new arguements\n     * @Context has admin, global_authority accounts.\n     * and zero-account Raffle, owner's nft ATA and global_authority's nft ATA\n     * and token mint address\n     * @param ticket_price: ticket price\n     * @param end_timestamp: the end time of raffle     \n     * @param max_entrants: entrants amount to take part in this raffle"
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
          "name": "raffle",
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
          "name": "ticketTokenProgram",
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
          "name": "raffleTokenAmount",
          "type": "u64"
        },
        {
          "name": "ticketPrice",
          "type": "u64"
        },
        {
          "name": "endTimestamp",
          "type": "i64"
        },
        {
          "name": "maxEntrants",
          "type": "u64"
        }
      ]
    },
    {
      "name": "updateRafflePeriod",
      "docs": [
        "* @dev Update Raffle Period\n     * @Context has admin and raffle accounts\n     * @param end_timestamp: new end_timestamp"
      ],
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "raffle",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "endTimestamp",
          "type": "i64"
        }
      ]
    },
    {
      "name": "buyTickets",
      "docs": [
        "* @dev Buy tickets functions\n     * @Context has buyer and raffle's account.\n     * global_authority and creator address and their reap token ATAs\n     * @param global_bump: global_authority's bump\n     * @param amount: the amount of the tickets"
      ],
      "accounts": [
        {
          "name": "buyer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "raffle",
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
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "creatorTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "adminTokenAccount",
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
        }
      ]
    },
    {
      "name": "revealWinner",
      "docs": [
        "* @dev Reaveal winner function\n     * @Context has buyer and raffle account address"
      ],
      "accounts": [
        {
          "name": "buyer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "raffle",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "claimReward",
      "docs": [
        "* @dev Claim reward function\n     * @Context has claimer and global_authority account\n     * raffle account and the nft ATA of claimer and global_authority.\n     * @param global_bump: the global_authority's bump"
      ],
      "accounts": [
        {
          "name": "claimer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raffle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "claimerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "srcTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMintAddress",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "globalBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "withdrawNft",
      "docs": [
        "* @dev Withdraw NFT function\n     * @Context has claimer and global_authority account\n     * raffle account and creator's nft ATA and global_authority's nft ATA\n     * @param global_bump: global_authority's bump"
      ],
      "accounts": [
        {
          "name": "claimer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raffle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "claimerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "srcTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMintAddress",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "globalBump",
          "type": "u8"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "globalPool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "superAdmin",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "rafflePool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "creator",
            "type": "publicKey"
          },
          {
            "name": "tokenProgram",
            "type": "publicKey"
          },
          {
            "name": "tokenMint",
            "type": "publicKey"
          },
          {
            "name": "tokenAmount",
            "type": "u64"
          },
          {
            "name": "ticketTokenProgram",
            "type": "publicKey"
          },
          {
            "name": "ticketPrice",
            "type": "u64"
          },
          {
            "name": "count",
            "type": "u64"
          },
          {
            "name": "noRepeat",
            "type": "u64"
          },
          {
            "name": "maxEntrants",
            "type": "u64"
          },
          {
            "name": "startTimestamp",
            "type": "i64"
          },
          {
            "name": "endTimestamp",
            "type": "i64"
          },
          {
            "name": "whitelisted",
            "type": "u64"
          },
          {
            "name": "winner",
            "type": "publicKey"
          },
          {
            "name": "entrants",
            "type": {
              "array": [
                "publicKey",
                1024
              ]
            }
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "MaxEntrantsTooLarge",
      "msg": "Max entrants is too large"
    },
    {
      "code": 6001,
      "name": "RaffleEnded",
      "msg": "Raffle has ended"
    },
    {
      "code": 6002,
      "name": "RaffleNotEnded",
      "msg": "Raffle has not ended"
    },
    {
      "code": 6003,
      "name": "InvalidPrizeIndex",
      "msg": "Invalid prize index"
    },
    {
      "code": 6004,
      "name": "EndTimeError",
      "msg": "Invalid new End time"
    },
    {
      "code": 6005,
      "name": "NoPrize",
      "msg": "No prize"
    },
    {
      "code": 6006,
      "name": "NotCreator",
      "msg": "You are not the Creator"
    },
    {
      "code": 6007,
      "name": "NotWinner",
      "msg": "You are not the Winnner"
    },
    {
      "code": 6008,
      "name": "OtherEntrants",
      "msg": "There are other Entrants"
    },
    {
      "code": 6009,
      "name": "InvalidCalculation",
      "msg": "Invalid calculation"
    },
    {
      "code": 6010,
      "name": "NotEnoughToken",
      "msg": "You don't have enough token"
    },
    {
      "code": 6011,
      "name": "NotEnoughSOL",
      "msg": "You don't have enough SOL"
    },
    {
      "code": 6012,
      "name": "NotEnoughTicketsLeft",
      "msg": "Not enough tickets left"
    },
    {
      "code": 6013,
      "name": "RaffleStillRunning",
      "msg": "Raffle is still running"
    },
    {
      "code": 6014,
      "name": "WinnersAlreadyDrawn",
      "msg": "Winner already drawn"
    },
    {
      "code": 6015,
      "name": "WinnerNotDrawn",
      "msg": "Winner not drawn"
    },
    {
      "code": 6016,
      "name": "InvalidRevealedData",
      "msg": "Invalid revealed data"
    },
    {
      "code": 6017,
      "name": "TokenAccountNotOwnedByWinner",
      "msg": "Ticket account not owned by winner"
    },
    {
      "code": 6018,
      "name": "TicketHasNotWon",
      "msg": "Ticket has not won"
    },
    {
      "code": 6019,
      "name": "UnclaimedPrizes",
      "msg": "Unclaimed prizes"
    },
    {
      "code": 6020,
      "name": "InvalidRecentBlockhashes",
      "msg": "Invalid recent blockhashes"
    },
    {
      "code": 6021,
      "name": "InvalidToReveal",
      "msg": "Invalid to reveal"
    }
  ]
};

export const IDL: Solucky = {
  "version": "0.1.0",
  "name": "solucky",
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
      "name": "createRaffle",
      "docs": [
        "* @dev Create new raffle with new arguements\n     * @Context has admin, global_authority accounts.\n     * and zero-account Raffle, owner's nft ATA and global_authority's nft ATA\n     * and token mint address\n     * @param ticket_price: ticket price\n     * @param end_timestamp: the end time of raffle     \n     * @param max_entrants: entrants amount to take part in this raffle"
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
          "name": "raffle",
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
          "name": "ticketTokenProgram",
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
          "name": "raffleTokenAmount",
          "type": "u64"
        },
        {
          "name": "ticketPrice",
          "type": "u64"
        },
        {
          "name": "endTimestamp",
          "type": "i64"
        },
        {
          "name": "maxEntrants",
          "type": "u64"
        }
      ]
    },
    {
      "name": "updateRafflePeriod",
      "docs": [
        "* @dev Update Raffle Period\n     * @Context has admin and raffle accounts\n     * @param end_timestamp: new end_timestamp"
      ],
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "raffle",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "endTimestamp",
          "type": "i64"
        }
      ]
    },
    {
      "name": "buyTickets",
      "docs": [
        "* @dev Buy tickets functions\n     * @Context has buyer and raffle's account.\n     * global_authority and creator address and their reap token ATAs\n     * @param global_bump: global_authority's bump\n     * @param amount: the amount of the tickets"
      ],
      "accounts": [
        {
          "name": "buyer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "raffle",
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
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "creatorTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "adminTokenAccount",
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
        }
      ]
    },
    {
      "name": "revealWinner",
      "docs": [
        "* @dev Reaveal winner function\n     * @Context has buyer and raffle account address"
      ],
      "accounts": [
        {
          "name": "buyer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "raffle",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "claimReward",
      "docs": [
        "* @dev Claim reward function\n     * @Context has claimer and global_authority account\n     * raffle account and the nft ATA of claimer and global_authority.\n     * @param global_bump: the global_authority's bump"
      ],
      "accounts": [
        {
          "name": "claimer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raffle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "claimerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "srcTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMintAddress",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "globalBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "withdrawNft",
      "docs": [
        "* @dev Withdraw NFT function\n     * @Context has claimer and global_authority account\n     * raffle account and creator's nft ATA and global_authority's nft ATA\n     * @param global_bump: global_authority's bump"
      ],
      "accounts": [
        {
          "name": "claimer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raffle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "claimerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "srcTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMintAddress",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "globalBump",
          "type": "u8"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "globalPool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "superAdmin",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "rafflePool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "creator",
            "type": "publicKey"
          },
          {
            "name": "tokenProgram",
            "type": "publicKey"
          },
          {
            "name": "tokenMint",
            "type": "publicKey"
          },
          {
            "name": "tokenAmount",
            "type": "u64"
          },
          {
            "name": "ticketTokenProgram",
            "type": "publicKey"
          },
          {
            "name": "ticketPrice",
            "type": "u64"
          },
          {
            "name": "count",
            "type": "u64"
          },
          {
            "name": "noRepeat",
            "type": "u64"
          },
          {
            "name": "maxEntrants",
            "type": "u64"
          },
          {
            "name": "startTimestamp",
            "type": "i64"
          },
          {
            "name": "endTimestamp",
            "type": "i64"
          },
          {
            "name": "whitelisted",
            "type": "u64"
          },
          {
            "name": "winner",
            "type": "publicKey"
          },
          {
            "name": "entrants",
            "type": {
              "array": [
                "publicKey",
                1024
              ]
            }
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "MaxEntrantsTooLarge",
      "msg": "Max entrants is too large"
    },
    {
      "code": 6001,
      "name": "RaffleEnded",
      "msg": "Raffle has ended"
    },
    {
      "code": 6002,
      "name": "RaffleNotEnded",
      "msg": "Raffle has not ended"
    },
    {
      "code": 6003,
      "name": "InvalidPrizeIndex",
      "msg": "Invalid prize index"
    },
    {
      "code": 6004,
      "name": "EndTimeError",
      "msg": "Invalid new End time"
    },
    {
      "code": 6005,
      "name": "NoPrize",
      "msg": "No prize"
    },
    {
      "code": 6006,
      "name": "NotCreator",
      "msg": "You are not the Creator"
    },
    {
      "code": 6007,
      "name": "NotWinner",
      "msg": "You are not the Winnner"
    },
    {
      "code": 6008,
      "name": "OtherEntrants",
      "msg": "There are other Entrants"
    },
    {
      "code": 6009,
      "name": "InvalidCalculation",
      "msg": "Invalid calculation"
    },
    {
      "code": 6010,
      "name": "NotEnoughToken",
      "msg": "You don't have enough token"
    },
    {
      "code": 6011,
      "name": "NotEnoughSOL",
      "msg": "You don't have enough SOL"
    },
    {
      "code": 6012,
      "name": "NotEnoughTicketsLeft",
      "msg": "Not enough tickets left"
    },
    {
      "code": 6013,
      "name": "RaffleStillRunning",
      "msg": "Raffle is still running"
    },
    {
      "code": 6014,
      "name": "WinnersAlreadyDrawn",
      "msg": "Winner already drawn"
    },
    {
      "code": 6015,
      "name": "WinnerNotDrawn",
      "msg": "Winner not drawn"
    },
    {
      "code": 6016,
      "name": "InvalidRevealedData",
      "msg": "Invalid revealed data"
    },
    {
      "code": 6017,
      "name": "TokenAccountNotOwnedByWinner",
      "msg": "Ticket account not owned by winner"
    },
    {
      "code": 6018,
      "name": "TicketHasNotWon",
      "msg": "Ticket has not won"
    },
    {
      "code": 6019,
      "name": "UnclaimedPrizes",
      "msg": "Unclaimed prizes"
    },
    {
      "code": 6020,
      "name": "InvalidRecentBlockhashes",
      "msg": "Invalid recent blockhashes"
    },
    {
      "code": 6021,
      "name": "InvalidToReveal",
      "msg": "Invalid to reveal"
    }
  ]
};
