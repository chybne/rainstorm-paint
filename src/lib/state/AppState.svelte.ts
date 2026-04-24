import Color from "$lib/utils/color";

class AppState {
    private selectedColor: Color = $state(new Color(0.6, 1.0, 1.0));

    getColor() {
        return this.selectedColor;
    }

    setColor(color: Color) {
        this.selectedColor = color;
    }
}

export const appState = new AppState();
