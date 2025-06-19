import { inventEntirelyNewTypeOfGuy } from 'tl:guy-fighter/host';

export function init() {
    inventEntirelyNewTypeOfGuy({
        name: "Guy who's made entirely of JavaScript",
        strength: 13,
        agility: 20,
        charisma: 11,
        battleCries: [
           "console.log('Time for DEATH!')",
            "let i_will_destroy_you = true;",
        ]
    });
}

export function getPluginName() {
    return 'Coffee Guy Plugin';
}