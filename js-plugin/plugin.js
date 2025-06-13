import { inventEntirelyNewTypeOfGuy } from 'tl:guy-fighter/guy-fighter-host';

export function init() {
    inventEntirelyNewTypeOfGuy({
        name: "Guy who's made entirely of JavaScript",
        strength: 13,
        agility: 20,
        charisma: 11,
    });
}

export function getPluginName() {
    return 'Coffee Guy Plugin';
}