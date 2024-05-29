const colors = {
    light: {
        "primary": "#0ea5e9",
        "secondary": "#a3e635",
        "accent": "#fb923c",
        "neutral": "#fdba74",
        "base-100": "#f3f4f6",
        "info": "#a5f3fc",
        "success": "#22c55e",
        "warning": "#fde047",
        "error": "#ef4444",
    },
    dark: {
        "primary": "#1d4ed8",
        "secondary": "#4d7c0f",
        "accent": "#a16207",
        "neutral": "#854d0e",
        "base-100": "#1c1917",
        "info": "#0891b2",
        "success": "#84cc16",
        "warning": "#eab308",
        "error": "#ef4444",
    }
}

const variables = {
    "--rounded-box": "0.5rem",          // border radius rounded-box utility class, used in card and other large boxes
    "--rounded-btn": "0.2rem",        // border radius rounded-btn utility class, used in buttons and similar element
    "--rounded-badge": "1.0rem",      // border radius rounded-badge utility class, used in badges and similar
    "--animation-btn": "0.25s",       // duration of animation when you click on button
    "--animation-input": "0.2s",      // duration of animation for inputs like checkbox, toggle, radio, etc
    "--btn-focus-scale": "0.95",      // scale transform of button when you focus on it
    "--border-btn": "1px",            // border width of buttons
    "--tab-border": "1px",            // border width of tabs
    "--tab-radius": "0.2rem",         // border radius of tabs
}


export default Object.fromEntries(
    Object.entries(colors).map(([key, value]) => [key, {...value, ...variables}])
)