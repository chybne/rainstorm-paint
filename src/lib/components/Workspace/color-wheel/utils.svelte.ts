

function createColorGradient(centerX: number, centerY: number, ctx: CanvasRenderingContext2D): CanvasGradient {
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

export function getAngleFromPoint(
    x: number,
    y: number,
    centerX: number,
    centerY: number
): number {
    const dx = x - centerX;
    const dy = y - centerY;

    let angle = Math.atan2(dy, dx); // -PI → PI
    angle = (angle + 2 * Math.PI) % (2 * Math.PI); // 0 → 2PI

    return angle;
}
export function angleToHue(angle: number): number {
    const hue = (angle / (2 * Math.PI)) * 360;
    return hue;
}
export function hueToAngle(hue: number): number {
    const angle = (hue / 360) * (2 * Math.PI);
    return angle;
}


export class ColorWheel {
    size: number;
    centerX: number;
    centerY: number;
    circleRadius: number;
    colorGradientRadius: number;
    wheelThickness: number;
    
    constructor(size: number) {
        this.size = size;
        this.wheelThickness = size * 0.1;
        this.centerX = size / 2;
        this.centerY = size / 2;
        this.circleRadius = size / 2 - 15;
        this.colorGradientRadius = this.circleRadius - (this.wheelThickness / 2) - 2;
    }

    private drawColorGradientWheel(ctx: CanvasRenderingContext2D) {
        const colorGradient = createColorGradient(this.centerX, this.centerY, ctx);

        ctx.beginPath();
        ctx.lineCap = 'butt';
        ctx.lineWidth = this.wheelThickness;
        ctx.strokeStyle = colorGradient;
        ctx.arc(this.centerX, this.centerY, this.colorGradientRadius, 0, 2 * Math.PI);
        ctx.stroke();
        ctx.closePath();
    }

    private drawWheelBackground(fillColor: string, ctx: CanvasRenderingContext2D) {
        ctx.beginPath();
        ctx.lineCap = 'butt';
        ctx.fillStyle = fillColor;
        ctx.arc(this.centerX, this.centerY, this.circleRadius, 0, 2 * Math.PI);
        ctx.fill();
        ctx.closePath();
    } 

    private drawColorSelector(hue: number, ctx: CanvasRenderingContext2D) {
        // 1. Convert the Hue (0-360) to Radians (0-2PI)
        const angle = hueToAngle(hue);

        // 2. Calculate the position using Polar coordinates
        // We use colorGradientRadius because that is the center line of the gradient ring
        const selectorX = this.centerX + this.colorGradientRadius * Math.cos(angle);
        const selectorY = this.centerY + this.colorGradientRadius * Math.sin(angle);

        // 3. Define the size of the selector handle
        // We make it slightly smaller than the wheel thickness so it fits nicely
        const selectorRadius = (this.wheelThickness / 2) - 5;

        ctx.beginPath();
        
        // Draw the circle
        ctx.arc(selectorX, selectorY, selectorRadius, 0, 2 * Math.PI);
        
        ctx.lineWidth = 7;
        ctx.strokeStyle = "#433c73"; 
        ctx.stroke();

        // Optional: Add a subtle stroke for better contrast
        ctx.lineWidth = 3;
        ctx.strokeStyle = "#FFFFFF"; 
        ctx.stroke();

        // Reset shadow to avoid affecting other drawing operations
        ctx.shadowColor = "transparent";
        ctx.shadowBlur = 0;
        ctx.shadowOffsetX = 0;
        ctx.shadowOffsetY = 0;

        ctx.closePath();
    }

    drawWheel(selectedHue: number, ctx: CanvasRenderingContext2D) {
        const fillColor = "#433c73"

        this.drawWheelBackground(fillColor, ctx);
        this.drawColorGradientWheel(ctx);
        this.drawColorSelector(selectedHue, ctx);
    }
}