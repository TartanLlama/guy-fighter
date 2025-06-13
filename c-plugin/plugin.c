#include "gen/guy_fighter_plugin.h"

void exports_guy_fighter_plugin_init() {
    tl_guy_fighter_guy_fighter_host_type_of_guy_t new_guy = {
        .strength = 12,
        .charisma = 10,
        .agility = 10,
    };
    guy_fighter_plugin_string_dup(&new_guy.name, "Guy who's secretly a dog and also made of nails");
    tl_guy_fighter_guy_fighter_host_invent_entirely_new_type_of_guy(&new_guy);
}

void exports_guy_fighter_plugin_get_plugin_name(guy_fighter_plugin_string_t *ret) {
    guy_fighter_plugin_string_dup(ret, "Deluxe Dog Plugin");
}