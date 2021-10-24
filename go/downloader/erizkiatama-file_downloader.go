// LANGUAGE: Go
// AUTHOR: M. Ezra Rizkiatama P.
// GITHUB: https://github.com/erizkiatama

// MODIFICATION; A. B. Clausen
// GITHUB: https://github.com/anbclausen

package main

import (
	"fmt"
	"io"
	"log"
	"math"
	"net/http"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"sync"
)

type progressWriter struct {
	currentSize    int64
	currentPercent float64
	Filename       string
	Number         int
	Size           int64
	TextHelper     []string
}

func (pw *progressWriter) Write(p []byte) (int, error) {
	n := len(p)
	pw.currentSize += int64(n)
	percent := getPercent(math.Ceil(float64(pw.currentSize) / float64(pw.Size) * 100))
	if percent != pw.currentPercent {
		if int64(percent)%10 == 0 {
			pw.currentPercent = percent
			pw.TextHelper[pw.Number] = fmt.Sprintf("Downloading %s... %.0f%% complete", pw.Filename, pw.currentPercent)
			pw.printProgress()
		}
	}

	return n, nil
}

func (pw *progressWriter) printProgress() {
	reset := ""
	progress := ""

	for i := 0; i < len(pw.TextHelper); i++ {
		reset += fmt.Sprintf("%s\n", strings.Repeat(" ", 40))
		progress += pw.TextHelper[i] + "\n"
	}
	fmt.Printf("%s\r%s", strings.Repeat("\033[F", len(pw.TextHelper)), reset)
	fmt.Printf("%s\r%s", strings.Repeat("\033[F", len(pw.TextHelper)), progress)
}

func download(urls []string) {
	var wg sync.WaitGroup

	totalURL := len(urls)
	textHelper := make([]string, totalURL)

	dir, err := os.Getwd()
	if err != nil {
		log.Fatal("Error when opening directory")
	}

	wg.Add(totalURL)

	fmt.Println("Download Started")
	fmt.Printf("%s", strings.Repeat("\n", totalURL))

	for i, url := range urls {
		number := i
		go func(url string) {
			defer wg.Done()
			tokens := strings.Split(url, "/")
			fileName := tokens[len(tokens)-1]

			fileLocation := filepath.Join(filepath.Dir(dir), fileName)

			output, err := os.Create(fileLocation)
			if err != nil {
				log.Fatal("Error while creating", fileName, "-", err)
			}
			defer output.Close()

			res, err := http.Get(url)
			if err != nil {
				log.Fatal("http get error: ", err)
			} else {
				defer res.Body.Close()
				size, _ := strconv.Atoi(res.Header.Get("Content-Length"))
				pw := &progressWriter{
					Size:       int64(size),
					Number:     number,
					Filename:   fileName,
					TextHelper: textHelper,
				}
				_, err = io.Copy(output, io.TeeReader(res.Body, pw))
				if err != nil {
					log.Fatal("Error while downloading", url, "-", err)
				}
			}
		}(url)
	}
	wg.Wait()
	fmt.Println("Download Done")
}

func main() {
	s := strconv.Itoa(1000)

	fmt.Printf("Res: %v", len(s)%2 == 0)
}

func getPercent(percent float64) float64 {
	if percent > 100 {
		return 100
	}
	return math.Ceil(percent/10) * 10
}
