export default class Color {
    constructor(
        private hue: number,
        private saturation: number,
        private value: number,
        private alpha: number,
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

    getAlpha() {
        return this.alpha;
    }

    toRGB() {
        let h = this.hue,
            s = this.saturation,
            v = this.value,
            a = this.alpha;

        let r = 0,
            g = 0,
            b = 0;

        let i = Math.floor(h * 6);
        let f = h * 6 - i;
        let p = v * (1 - s);
        let q = v * (1 - f * s);
        let t = v * (1 - (1 - f) * s);

        switch (i % 6) {
            case 0:
                r = v;
                g = t;
                b = p;
                break;
            case 1:
                r = q;
                g = v;
                b = p;
                break;
            case 2:
                r = p;
                g = v;
                b = t;
                break;
            case 3:
                r = p;
                g = q;
                b = v;
                break;
            case 4:
                r = t;
                g = p;
                b = v;
                break;
            case 5:
                r = v;
                g = p;
                b = q;
                break;
        }

        return [r, g, b, a];
    }
}
