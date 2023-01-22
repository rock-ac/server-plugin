#if defined _INC_ROCK_ANTICHEAT
    #endinput
#endif
#define _INC_ROCK_ANTICHEAT

native RAC_SetAccessToken(const token[]);
native RAC_SetUserSessionStatus(const username[], bool: status);

native RAC_BanUser(const username[], const reason[]);
native RAC_UnbanUser(const username[]);