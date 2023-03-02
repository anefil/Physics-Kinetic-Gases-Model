import * as d3 from "d3";

const xMax = 200;
const yMax = 350;
export function initChart(){
    const margin = { top: 10, right: 50, bottom: 50, left: 50 },
        width = 450 - margin.left - margin.right,
        height = 400 - margin.top - margin.bottom;

    const svg = d3.select("#root").attr("width", width + margin.left + margin.right)
        .attr("height", height + margin.top + margin.bottom)
        .append("g")
        .attr("transform", "translate(" + margin.left + "," + margin.top + ")");

// Define chart area
    svg
        .append("clipPath")
        .attr("id", "chart-area")
        .append("rect")
        .attr("x", 0)
        .attr("y", 0)
        .attr("width", width)
        .attr("height", height)

    // Add Axes

    let xScale = d3.scaleLinear([0, xMax], [0, width])
    let yScale = d3.scaleLinear([0, yMax], [height, 0])

    let xAxis = d3.axisBottom(xScale)
    let yAxis = d3.axisLeft(yScale)
    svg.append("g")
        .attr("transform", `translate(0,${height})`)
        .call(xAxis)
    svg.append("g")
        .attr("transform", `translate(0,0)`)
        .call(yAxis)

// Axes label
    svg.append("text")
        .attr("class", "x label")
        .attr("text-anchor", "end")
        .attr("x", width / 2 + 5)
        .attr("y", height + 35)
        .text("t");

    svg.append("text")
        .attr("class", "y label")
        .attr("text-anchor", "end")
        .attr("y", -35)
        .attr("x", -height / 2)
        .attr("transform", "rotate(-90)")
        .html("P")
}

// I need to update the chart with new data
// I have only Y values and X values are the time
// write function to update the chart

export function updateChart(data) {

    const margin = { top: 10, right: 50, bottom: 50, left: 50 },
        width = 450 - margin.left - margin.right,
        height = 400 - margin.top - margin.bottom;


    const svg = d3.select("#root>g");
    let xScale = d3.scaleLinear([0, xMax], [0, width])
    let yScale = d3.scaleLinear([0, yMax], [height, 0])


    // Add point into chart
    svg.append("circle")
        .attr("cx", xScale(data[0]))
        .attr("cy", yScale(data[1]))
        .attr("r", 2)
        .attr("fill", "red")
        .attr("clip-path", "url(#chart-area)")


}