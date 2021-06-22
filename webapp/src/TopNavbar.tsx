import React from "react"
import {Button, Navbar} from "react-bootstrap"

export const TopNavbar = () => {
    return <Navbar bg="dark" className="text-white border-bottom d-flex flex-wrap align-items-center justify-content-center justify-content-md-between py-3 mb-4">
        <a href="/" className="d-flex align-items-center col-md-3 mb-2 mb-md-0 text-white text-decoration-none">
            <img src="https://i.ytimg.com/vi/bDByGe7FgEQ/hqdefault.jpg" height="50" width="50" />
        </a>
        <ul className="nav col-12 col-md-auto mb-2 justify-content-center mb-md-0">
            <li><a href="/" className="nav-link px-2 text-white">Home</a></li>
            <li><a href="/rankings" className="nav-link px-2 text-white">Rankings</a></li>
            <li><a href="/spec" className="nav-link px-2 text-white">Spec</a></li>
            <li><a href="/faq" className="nav-link px-2 text-white">FAQs</a></li>
            <li><a href="/about" className="nav-link px-2 text-white">About</a></li>
        </ul>
        <div className="col-md-3 text-end">
            <span className="me-2">z5555555</span>
            <Button variant={true ? "danger" : "warning"} className="me-2">Logout</Button>
        </div>
    </Navbar>
}