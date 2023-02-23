// variable for the namespace 
const svgns = "http://www.w3.org/2000/svg";
// make a simple rectangle


export class Rectangle {

    constructor(height, width,x=0, y=0, color="#F76902", opacity=1) {
        this.element = document.createElementNS(svgns, "rect"); 
        this.setHeight(height)
            .setWidth(width)
            .setPos(x, y)
            .setColor(color)
            .setOpacity(opacity);
    }

    getElement() {
        return this.element;
    }

    setWidth(w) {
        this.element.setAttribute("width", w);
        return this;
    }

    setHeight(h) {
        this.element.setAttribute("height", h);
        return this;
    }

    setPos(x, y) {
        this.element.setAttribute("x", x);
        this.element.setAttribute("y", y);
        return this;
    }

    setColor(c) {
        this.element.setAttribute("fill", c);
        return this;
    }


    rotate(d, x, y) {
        this.element.setAttribute("transform", `rotate(${d} ${x} ${y})`);
        return this;
    }

    setOpacity(o) {
        this.element.setAttribute("opacity", o);
        return this;
    }
}