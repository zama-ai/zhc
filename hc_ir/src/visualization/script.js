function getRandomColor() {
    return '#' + Math.floor(Math.random() * 16777215).toString(16).padStart(6, '0');
}

document.querySelectorAll('.interactive-path').forEach(path => {
    // Store original values
    const originalStroke = path.getAttribute('stroke');
    const originalStrokeWidth = path.getAttribute('stroke-width');

    path.addEventListener('click', function() {
        if (this.dataset.modified === 'true') {
            // Reset to original
            this.setAttribute('stroke', originalStroke);
            this.setAttribute('stroke-width', originalStrokeWidth);
            this.dataset.modified = 'false';
        } else {
            // Apply random color and bigger width
            this.setAttribute('stroke', getRandomColor());
            this.setAttribute('stroke-width', '3');
            this.dataset.modified = 'true';
        }
    });
});

// Zoom functionality
const svg = document.querySelector('svg');
const originalWidth = parseFloat(svg.getAttribute('width'));
const originalHeight = parseFloat(svg.getAttribute('height'));
let viewBox = { x: 0, y: 0, width: originalWidth, height: originalHeight };

function updateViewBox() {
    svg.setAttribute('viewBox', `${viewBox.x} ${viewBox.y} ${viewBox.width} ${viewBox.height}`);
}

// Add wheel event listener for zoom
document.addEventListener('wheel', function(e) {
    if (e.metaKey || e.ctrlKey) { // cmd on Mac, ctrl on Windows/Linux
        e.preventDefault();

        const rect = svg.getBoundingClientRect();
        const mouseX = e.clientX - rect.left;
        const mouseY = e.clientY - rect.top;

        // Convert mouse position to SVG coordinates
        const svgMouseX = viewBox.x + (mouseX / rect.width) * viewBox.width;
        const svgMouseY = viewBox.y + (mouseY / rect.height) * viewBox.height;

        const zoomFactor = e.deltaY > 0 ? 1.1 : 0.9; // Inverted for viewBox
        const newWidth = Math.min(Math.max(viewBox.width * zoomFactor, originalWidth * 0.1), originalWidth * 10);
        const newHeight = Math.min(Math.max(viewBox.height * zoomFactor, originalHeight * 0.1), originalHeight * 10);

        // Adjust viewBox to zoom towards mouse position
        const widthChange = newWidth - viewBox.width;
        const heightChange = newHeight - viewBox.height;

        viewBox.x -= widthChange * (mouseX / rect.width);
        viewBox.y -= heightChange * (mouseY / rect.height);
        viewBox.width = newWidth;
        viewBox.height = newHeight;

        updateViewBox();
    }
}, { passive: false });
