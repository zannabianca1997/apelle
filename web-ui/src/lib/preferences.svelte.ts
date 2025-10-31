export interface Preferences {
    volume: number;
    playFromHere: boolean;
    autoNext: boolean;
}

export const preferences: Preferences = $state({
    volume: 1.0,
    playFromHere: false,
    autoNext: true
});
