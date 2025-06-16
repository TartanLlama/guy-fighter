#include "gen/guy_fighter_plugin.h"
#include <stdlib.h>

void exports_guy_fighter_plugin_init() {
    tl_guy_fighter_guy_fighter_host_type_of_guy_t new_guy = {
        .strength = 12,
        .charisma = 10,
        .agility = 10,
    };
    guy_fighter_plugin_string_dup(&new_guy.name, "Guy who's secretly a dog and also made of nails");
    guy_fighter_plugin_list_string_t battle_cries;
    battle_cries.len = 2;
    battle_cries.ptr = (guy_fighter_plugin_string_t *) malloc(battle_cries.len * sizeof(guy_fighter_plugin_string_t));
    guy_fighter_plugin_string_dup(&battle_cries.ptr[0], "Woof!");
    guy_fighter_plugin_string_dup(&battle_cries.ptr[1], "Bark bark!");
    new_guy.battle_cries = battle_cries;
    tl_guy_fighter_guy_fighter_host_invent_entirely_new_type_of_guy(&new_guy);
}

void exports_guy_fighter_plugin_get_plugin_name(guy_fighter_plugin_string_t *ret) {
    guy_fighter_plugin_string_dup(ret, "Deluxe Dog Plugin");
}