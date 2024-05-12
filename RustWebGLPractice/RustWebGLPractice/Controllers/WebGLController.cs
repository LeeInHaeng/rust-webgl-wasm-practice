using Microsoft.AspNetCore.Mvc;
using RustWebGLPractice.Models;
using System.Diagnostics;

namespace RustWebGLPractice.Controllers
{
    public class WebGLController : Controller
    {
        public IActionResult CanvasCreate()
        {
            return View();
        }

        public IActionResult DrawTriangle()
        {
            return View();
        }

        public IActionResult DrawPoint()
        {
            return View();
        }
    }
}