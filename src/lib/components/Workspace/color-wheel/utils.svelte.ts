import Color from "$lib/utils/color";

function createColorGradient(
    centerX: number,
    centerY: number,
    ctx: CanvasRenderingContext2D,
): CanvasGradient {
    const gradient = ctx.createConicGradient(0, centerX, centerY);
    gradient.addColorStop(0, "cyan");
    gradient.addColorStop(1 / 6, "blue");
    gradient.addColorStop(2 / 6, "magenta");
    gradient.addColorStop(3 / 6, "red");
    gradient.addColorStop(4 / 6, "yellow");
    gradient.addColorStop(5 / 6, "lime");
    gradient.addColorStop(1, "cyan");

    return gradient;
}

export function getAngleFromPoint(dx: number, dy: number): number {
    let angle = Math.atan2(dy, dx); // -PI → PI
    angle = (angle + 2 * Math.PI) % (2 * Math.PI); // 0 → 2PI

    return angle;
}
export function angleToHue(angle: number): number {
    const hue = (angle / (2 * Math.PI)) * 360;

    let adjustedHue = (hue + 180) % 360;

    adjustedHue /= 360;
    return adjustedHue;
}
export function hueToAngle(hue: number): number {
    hue *= 360;
    let adjustedHue = (hue + 180) % 360;

    const angle = (adjustedHue / 360) * (2 * Math.PI);
    return angle;
}

/**
 * Converts HSV to RGB.
 * @param h Hue (0 - 1)
 * @param s Saturation (0 - 1)
 * @param v Value (0 - 1)
 * @returns [r, g, b] in the range 0 - 255
 */
function hsvToRgb(h: number, s: number, v: number): [number, number, number] {
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

    return [Math.round(r * 255), Math.round(g * 255), Math.round(b * 255)];
}

enum SelectingPhase {
    IsSelectingHue,
    IsSelectingColor,
    None,
}

export class ColorWheel {
    size: number;
    centerX: number;
    centerY: number;
    circleRadius: number;
    colorGradientRadius: number;
    wheelThickness: number;
    innerCircleRadius: number;
    selectPhase: SelectingPhase;

    constructor(size: number) {
        this.size = size;
        this.wheelThickness = size * 0.07;
        this.centerX = size / 2;
        this.centerY = size / 2;
        this.circleRadius = size / 2 - 15;
        this.colorGradientRadius =
            this.circleRadius - this.wheelThickness / 2 - 2;
        this.innerCircleRadius = this.circleRadius - this.wheelThickness - 4;

        this.selectPhase = SelectingPhase.None;
    }

    private drawColorGradientWheel(ctx: CanvasRenderingContext2D) {
        const colorGradient = createColorGradient(
            this.centerX,
            this.centerY,
            ctx,
        );

        ctx.beginPath();
        ctx.lineCap = "butt";
        ctx.lineWidth = this.wheelThickness;
        ctx.strokeStyle = colorGradient;
        ctx.arc(
            this.centerX,
            this.centerY,
            this.colorGradientRadius,
            0,
            2 * Math.PI,
        );
        ctx.stroke();
        ctx.closePath();
    }

    private drawWheelBackground(
        fillColor: string,
        ctx: CanvasRenderingContext2D,
    ) {
        ctx.beginPath();
        ctx.lineCap = "butt";
        ctx.fillStyle = fillColor;
        ctx.arc(this.centerX, this.centerY, this.circleRadius, 0, 2 * Math.PI);
        ctx.fill();
        ctx.closePath();
    }

    private drawHueSelectorBar(hue: number, ctx: CanvasRenderingContext2D) {
        const angle = hueToAngle(hue);

        const innerR = this.colorGradientRadius - this.wheelThickness / 2;
        const outerR = this.colorGradientRadius + this.wheelThickness / 2;

        const rX = Math.cos(angle);
        const rY = Math.sin(angle);

        // Tangent (perpendicular) direction
        const tX = -rY;
        const tY = rX;

        const offset = 5;
        const cx = this.centerX;
        const cy = this.centerY;

        const drawLines = () => {
            ctx.beginPath();

            // line 1
            ctx.moveTo(
                cx + rX * innerR + tX * offset,
                cy + rY * innerR + tY * offset,
            );
            ctx.lineTo(
                cx + rX * outerR + tX * offset,
                cy + rY * outerR + tY * offset,
            );

            // line 2
            ctx.moveTo(
                cx + rX * innerR - tX * offset,
                cy + rY * innerR - tY * offset,
            );
            ctx.lineTo(
                cx + rX * outerR - tX * offset,
                cy + rY * outerR - tY * offset,
            );

            ctx.stroke();
        };

        // ---- border ----
        ctx.strokeStyle = "rgba(0,0,0,1)";
        ctx.lineWidth = 4;
        ctx.lineCap = "butt";
        drawLines();

        // ---- inner line ----
        ctx.strokeStyle = "rgba(255,255,255,1)";
        ctx.lineWidth = 1;
        drawLines();
    }

    private drawHueSelector(hue: number, ctx: CanvasRenderingContext2D) {
        // 1. Convert the Hue (0-360) to Radians (0-2PI)
        const angle = hueToAngle(hue);

        const selectorX =
            this.centerX + this.colorGradientRadius * Math.cos(angle);
        const selectorY =
            this.centerY + this.colorGradientRadius * Math.sin(angle);

        const selectorRadius = this.wheelThickness / 2 - 2;

        ctx.beginPath();

        // Draw the circle
        ctx.arc(selectorX, selectorY, selectorRadius, 0, 2 * Math.PI);

        ctx.lineWidth = 3;
        ctx.strokeStyle = "#433c73";
        ctx.stroke();

        ctx.lineWidth = 1;
        ctx.strokeStyle = "#FFFFFF";
        ctx.fillStyle = `hsl(${hue}, 100%, 50%)`;
        ctx.fill();
        ctx.stroke();

        ctx.shadowColor = "transparent";
        ctx.shadowBlur = 0;
        ctx.shadowOffsetX = 0;
        ctx.shadowOffsetY = 0;

        ctx.closePath();
    }

    private drawColorSelector(
        selectedHue: number,
        ctx: CanvasRenderingContext2D,
    ) {
        let dpr = window.devicePixelRatio;

        let size = 2 * this.innerCircleRadius * Math.sqrt(0.5) * dpr;

        let imageData = ctx.createImageData(size, size);

        let ptr = 0;
        for (let row = 0; row < imageData.height; row++) {
            for (let col = 0; col < imageData.width; col++) {
                let value = (imageData.height - row) / imageData.height;
                let saturation = col / imageData.width;

                let color = hsvToRgb(selectedHue, saturation, value);
                imageData.data[ptr] = color[0];
                imageData.data[ptr + 1] = color[1];
                imageData.data[ptr + 2] = color[2];
                imageData.data[ptr + 3] = 255;

                ptr += 4;
            }
        }

        ctx.putImageData(
            imageData,
            this.centerX * dpr - size / 2,
            this.centerY * dpr - size / 2,
        );
    }

    private drawColorSelectorCircle(
        selectedColor: Color,
        ctx: CanvasRenderingContext2D,
    ) {
        const size = 2 * this.innerCircleRadius * Math.sqrt(0.5);
        const startX = this.centerX - size / 2;
        const startY = this.centerY - size / 2;

        let x = selectedColor.getSaturation() * size + startX;
        let y = size - selectedColor.getValue() * size + startY;

        ctx.beginPath();

        ctx.arc(x, y, 4, 0, 2 * Math.PI);
        ctx.lineWidth = 3;
        ctx.strokeStyle = "#433c73";
        ctx.stroke();

        // Draw the circle
        ctx.arc(x, y, 4, 0, 2 * Math.PI);
        ctx.lineWidth = 1;
        ctx.strokeStyle = "#FFFFFF";
        ctx.stroke();

        ctx.closePath();
    }

    drawWheel(selectedColor: Color, ctx: CanvasRenderingContext2D) {
        const fillColor = "#433c73";

        this.drawWheelBackground(fillColor, ctx);
        this.drawColorSelector(selectedColor.getHue(), ctx);
        this.drawColorSelectorCircle(selectedColor, ctx);
        this.drawColorGradientWheel(ctx);
        this.drawHueSelectorBar(selectedColor.getHue(), ctx);
    }

    handleWidgetClick(x: number, y: number, prevColor: Color) {
        let centerX = this.size / 2;
        let centerY = this.size / 2;

        let dx = x - centerX;
        let dy = y - centerY;
        let distance = Math.sqrt(dx * dx + dy * dy);

        console.log(
            "distance",
            distance,
            "innerCircleRadius",
            this.innerCircleRadius,
            "circleRadius",
            this.circleRadius,
        );
        // Check if click is within the hue selector donut
        if (distance > this.innerCircleRadius && distance < this.circleRadius) {
            this.selectPhase = SelectingPhase.IsSelectingHue;

            const angle = getAngleFromPoint(dx, dy);

            let hue = angleToHue(angle);

            return new Color(
                hue,
                prevColor.getSaturation(),
                prevColor.getValue(),
            );
        }

        const size = 2 * this.innerCircleRadius * Math.sqrt(0.5);
        const startX = this.centerX - size / 2;
        const startY = this.centerY - size / 2;

        // check if
        if (
            x > startX &&
            x < startX + size &&
            y > startY &&
            y < startY + size
        ) {
            this.selectPhase = SelectingPhase.IsSelectingColor;
            let rangeX = (x - startX) / size;
            let rangeY = 1 - (y - startY) / size;

            console.log(`clicked inside of square ${rangeX} ${rangeY}`);
            return new Color(prevColor.getHue(), rangeX, rangeY);
        }

        return null;
    }

    handleWidgetClickMove(x: number, y: number, prevColor: Color) {
        if (this.selectPhase === SelectingPhase.IsSelectingHue) {
            const centerX = this.size / 2;
            const centerY = this.size / 2;

            const dx = x - centerX;
            const dy = y - centerY;

            const angle = getAngleFromPoint(dx, dy);
            let hue = angleToHue(angle);

            console.log("Selected color:", hue);
            return new Color(
                hue,
                prevColor.getSaturation(),
                prevColor.getValue(),
            );
        }

        if (this.selectPhase === SelectingPhase.IsSelectingColor) {
            const size = 2 * this.innerCircleRadius * Math.sqrt(0.5);
            const startX = this.centerX - size / 2;
            const startY = this.centerY - size / 2;

            x = Math.min(Math.max(x, startX), startX + size);
            y = Math.min(Math.max(y, startY), startY + size);

            let rangeX = (x - startX) / size;
            let rangeY = 1 - (y - startY) / size;

            console.log(`moved inside of square ${rangeX} ${rangeY}`);
            return new Color(prevColor.getHue(), rangeX, rangeY);
        }
    }

    handleWidgetClickUp() {
        this.selectPhase = SelectingPhase.None;
    }
}
