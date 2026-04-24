export default class Color {
    constructor(
        private hue: number,
        private saturation: number,
        private value: number,
    ) {}

    getHue() {
        return this.hue;
    }

    getValue() {
        return this.value;
    }

    getSaturation() {
        return this.saturation;
    }
}
