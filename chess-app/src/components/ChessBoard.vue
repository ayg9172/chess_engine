<script>
import { Rectangle } from "../logic/rectangle";
import axios from "axios";
export default {
  data() {

    return {
      fen: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
      turn: 'white',
      clickedSquare: null,
      board: null,
      message: 'Hello World!',
      isWaiting: false,
      STARTING_BOARD: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
      depth: 4 ,
    }
  },
  mounted() {
    this.update();
  },
  methods: { 

    is_numeric (str) {
    return /^\d+$/.test(str);
  },
    getSVG (fenCharacter) {
    let mappings = {
        p: "http://127.0.0.1:3154/p.svg",
        n: "http://127.0.0.1:3154/n.svg",
        b: "http://127.0.0.1:3154/b.svg",
        r: "http://127.0.0.1:3154/r.svg",
        q: "http://127.0.0.1:3154/q.svg",
        k: "http://127.0.0.1:3154/k.svg",
        P: "http://127.0.0.1:3154/P.svg",
        N: "http://127.0.0.1:3154/N.svg",
        B: "http://127.0.0.1:3154/B.svg",
        R: "http://127.0.0.1:3154/R.svg",
        Q: "http://127.0.0.1:3154/Q.svg",
        K: "http://127.0.0.1:3154/K.svg",
    }
    return mappings[fenCharacter];
},

    setBoard () {
      this.board = [
        [null, null, null, null, null, null, null, null], 
        [null, null, null, null, null, null, null, null], 
        [null, null, null, null, null, null, null, null],
        [null, null, null, null, null, null, null, null],
        [null, null, null, null, null, null, null, null],
        [null, null, null, null, null, null, null, null],
        [null, null, null, null, null, null, null, null],
        [null, null, null, null, null, null, null, null],
      ]
      for (let row = 0; row < 8; row++) {
        for (let col = 0; col < 8; col++) { 

          const size = 100;
          const opacity = 1;
          const rectangle = this.createRectangle(
            size, 
            size, 
            col * size,  
            row * size, 
            this.getColor(row, col), 
            opacity
          );
          this.board[row][col] = rectangle;
          const elm = rectangle.getElement();
          elm.onclick = () => {
            this.handleClick(row, col)
          }
        }
        
      }


    },
    setPieces (fen, board)  {

let col = 0;

let rows = fen.split("/");

for (let row = 0; row < 8; row++) {
  col = 0;

  for (let ch of rows[row]) {
    if (ch === " ") break;
    if (this.is_numeric(ch)) {
      col += Number(ch);
    } else {

      console.log(`${ch} ${row} ${col}`);
      const svgns = "http://www.w3.org/2000/svg";
      let elm = document.createElementNS(svgns, "image");
      elm.setAttribute("href", this.getSVG(ch));
      elm.setAttribute("x", String(100 * col));
      elm.setAttribute("y", String(100 * row));
      elm.setAttribute("width", String(100));
      elm.setAttribute("height", String(100));
      elm.onclick = () => {
        // todo why the heck does this work? but col does not?
        let c = Number(elm.getAttribute("x")) / 100;
        console.log(`Piece clicked ${ch} ${row} ${c}`)
        this.handleClick(row, c)
      }

      this.getAnchor().appendChild(elm);
      col += 1;
    }
  }
}


return board;
},
    clearBoard () {
    for (let row = 0; row < 8; row++) {
        for (let col = 0; col < 8; col++) {
            while(this.getAnchor().firstChild) {
              this.getAnchor().removeChild(this.getAnchor().firstChild);
            }
        }
    }
  },

  update() {
    this.clearBoard();
    this.setBoard();
    this.setPieces(this.fen, this.board);

  },
  updateDepth() {
    let slider = document.getElementById("depth");
    this.depth = slider.value;
  },


    handleClick(row, col) {
      if (this.isWaiting) {
        console.log("Waiting for opponent...")
        return;
      }
      console.log(`Processing click: ${row} ${col}`)
      this.board[row][col].setOpacity(0);
      if (this.clickedSquare) {
        const previousRow = this.clickedSquare.row;
        const previousCol = this.clickedSquare.col;
        this.clickedSquare = { row, col }

        if (previousRow !== row || previousCol !== col) {
          this.board[previousRow][previousCol].setOpacity(1);
          
          // MASSIVE TODO WE"RE SWITCHING COL ROW HERE
          this.tryMove(previousRow, previousCol, row, col);
        }
      }
      this.clickedSquare = { row, col }


    },
    

    tryMove(startRow, startCol, endRow, endCol) {
      this.isWaiting = true;

      let reqObj = {
        fen: this.fen,
        startRow,
        startCol,
        endRow,
        endCol,
        promotion: "None",
      };
      console.log(reqObj);
      axios.post("http://127.0.0.1:3131/requestMove", {
        fen: this.fen,
        startRow,
        startCol,
        endRow,
        endCol,
         depth: Number(this.depth),
        promotion: "None",
      }).then( (res, err) => {
        this.message = res.data.message;
        this.isWaiting = false;

        
        console.log("Response received")
        if (res.data.isLegal === false) {
          return;
        }
        this.fen = res.data.fen;
        this.update();
      });
    },


    getColor(row, col) {
      if (( row + (col % 2)) % 2) {
        return "#769656"
      }
      return "#eeeed2"
    },
    getAnchor() {
      return this.$refs.anchor;
    },
    createRectangle(height, width, x, y, color, opacity) {
      let obj = new Rectangle(height, width, x, y, color, opacity);
      let elm = obj.getElement();

      let parentElement = this.getAnchor();

      parentElement.appendChild(elm);

  
      return obj;
    },
    getStatus() {
      
    }
  }

}


</script>

<template>

  <div ref="boardContainer">
    <svg ref="anchor" :height="800" :width="800" xmlns:xlink="http://www.w3.org/1999/xlink"> </svg>
  </div>
  {{ message }}
<br>
<br>
<div class="slidecontainer">
  Depth: {{ depth }}
  <input @input="updateDepth" type="range" min="1" max="14" value="3" class="slider" id="depth">
</div>
<h1 v-if="isWaiting"> ai is thinking... </h1>
</template>

<style scoped>
body, html {
  height: 100%;
}

.checker {
  fill: #ffe9c5;
}
</style>