var btn = document.getElementById("generate");
function generateFlowchart(arr) {
  if (arr.length !== 0) {
    var DST = "st=>start: " + arr[0] + "\n";
    if (arr.length > 1) {
      DST += "e=>end: " + arr[arr.length-1] + "\n";
      }
    if (arr.length > 2) {
        var ids = [];
        for (var i = 1; i < arr.length-1; i++) {
          let id = "o" + i;
          DST += id + "=>operation: " + arr[i] + "\n";
          ids.push(id);
        }
      }
    DST += "st->";
    ids.forEach(e=>{DST += e+"->"});
    DST += "e\n";
    console.log(DST);
    var diagram = flowchart.parse(DST);
    diagram.drawSVG('draw');
  }
}

(btn.onclick = function () {generateFlowchart(["hello", "e", "bye"])});
// (btn.onclick = function () {
//   var diagram = flowchart.parse(`
//     st=>start: Start:>http://www.google.com[blank]
//     e=>end:>http://www.google.com
//     op1=>operation: My Operation
//     sub1=>subroutine: My Subroutine
//     cond=>condition: Yes
//     or No?:>http://www.google.com
//     io=>inputoutput: catch something...
//     para=>parallel: parallel tasks
//     in=>input: some in
//     out=>output: some out

//     st->op1->cond
//     cond(yes)->io->e
//     cond(no)->para
//     para(path1, bottom)->sub1(right)->op1
//     para(path2, top)->op1
//     para(path3, right)->in->out->e
//     `);
//   diagram.drawSVG('draw');
// });
