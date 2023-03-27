#if defined _INC_ROCK_ANTICHEAT
    #endinput
#endif
#define _INC_ROCK_ANTICHEAT

// * Sets the access token
// @param `const token[]` - given to the owner of the active server in RockCP
// * Example: RAC_SetAccessToken("your-token-here");
native RAC_SetAccessToken(const token[]);

// * Sets the status of the user (player) session
// @param `const username[]` - player name, which is specified in the SA-MP client and determined when connecting to the server
// @param `bool: status` - use "true" only after authorization and "false" after the player disconnects from the server
// * Example: RAC_SetUserSessionStatus("savvin", true);
native RAC_SetUserSessionStatus(const username[], bool: status);

// * Permanently blocks the user based on the characteristics of his equipment (designed to combat cheaters)
// @param `const username[]` - player name, which is specified in the SA-MP client and determined when connecting to the server
// @param `const reason[]` - reason shown only for the server owner in RockCP
// * Example: RAC_BanUser("savvin", "Aim-bot");
native RAC_BanUser(const username[], const reason[]);

// * Unblocks the user
// @param `const username[]` - player name, which is specified in the SA-MP client and determined when connecting to the server
// * Example: RAC_UnbanUser("savvin");
native RAC_UnbanUser(const username[]);