require('./index.html'); // Webpack hot reload for index.html 
import { TicTacToe } from './pkg';

const ticTacToe = TicTacToe.new(6, 3, 1);

const handleCellClick = (row, col) => {
  ticTacToe.make_move(row, col, ticTacToe.whos_turn());
  renderUI();
}

const getRowDivBase = () => {
  let rowDiv = document.createElement("DIV");
  rowDiv.classList.add('row');
  return rowDiv;
}

const getCellButton = (cell, row, col) => {
  let button = document.createElement('BUTTON');
  button.textContent = cell;
  button.setAttribute('data-row', row);
  button.setAttribute('data-col', col);
  button.addEventListener('click', (e) => {
    handleCellClick(
      Number(e.target.getAttribute('data-row')),
      Number(e.target.getAttribute('data-col')));
  });
  return button;
}

const setInfoText = (text) => {
  document.querySelector("#info").textContent = text;
}

const setTurnText = () => {
  setInfoText(`Turn: ${ticTacToe.get_turn_mark()}`);
}

const setWinnerText = () => {
  setInfoText(`Winner is: ${ticTacToe.get_winner()}`);
}

const getBoardElement = () => {
  return document.querySelector('#board');
}
const cleanBoard = () => {
  getBoardElement().innerHTML = '';
}

const createBoard = () => {
  let row = 0;
  let col = 0;
  let rowDiv = getRowDivBase();
  let boardCells = ticTacToe.get_board().split('');
  for (let i = 0; i < boardCells.length; i++) {
    let cell = boardCells[i];
    if (cell !== '\n') {
      rowDiv.append(getCellButton(cell, row, col));
      col += 1;
    } else {
      getBoardElement().append(rowDiv);
      rowDiv = getRowDivBase();
      row += 1;
      col = 0;
    }
  }
}

const renderUI = () => {
  if (ticTacToe.get_winner() !== ' ') {
    setWinnerText();
    cleanBoard();
  } else {
    setTurnText();
    cleanBoard();
    createBoard();
  }
}

renderUI();
