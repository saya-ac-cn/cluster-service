import {BrowserRouter, Route, Routes} from 'react-router-dom'
import {Suspense} from "react";
import Home from "./pages/home";
function App() {
  return (
    <BrowserRouter>
        <Suspense>
            <Routes>
                <Route path='/' element={<Home/>}/>
            </Routes>
        </Suspense>
    </BrowserRouter>
  );
}

export default App;
