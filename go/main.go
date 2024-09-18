package main

import (
	"encoding/csv"
	"fmt"
	"os"
	"strconv"
	"time"

	"github.com/RoaringBitmap/roaring/v2"
	humanize "github.com/dustin/go-humanize"
)

func main() {
	input, err := os.Open("../group_members.csv")
	if err != nil {
		panic(err)
	}

	reader := csv.NewReader(input)

	header, err := reader.Read()
	if err != nil {
		panic(err)
	}

	if len(header) != 2 {
		panic("bad header")
	}

	fmt.Printf("csv header: %v\n", header)

	start := time.Now()

	bitmaps := map[int]*roaring.Bitmap{}

	total := int64(0)
	skipped := int64(0)

	for {
		record, err := reader.Read()
		if err != nil {
			break
		}

		if len(record) != 2 || record[0] == "" || record[1] == "" {
			// fmt.Printf("skipping bad record: %v\n", record)
			skipped++
			continue
		}

		total++

		groupID, err := strconv.Atoi(record[0])
		if err != nil {
			panic(err)
		}
		memberID, err := strconv.Atoi(record[1])
		if err != nil {
			panic(err)
		}

		bitmap, ok := bitmaps[groupID]
		if !ok {
			bitmap = roaring.New()
			bitmaps[groupID] = bitmap
		}
		bitmap.Add(uint32(memberID))
	}

	totalSize := uint64(0)

	for groupID, bitmap := range bitmaps {
		bitmap.RunOptimize()
		fmt.Printf("group %d has %s members size %s\n", groupID, humanize.Comma(int64(bitmap.GetCardinality())), humanize.Bytes(bitmap.GetSizeInBytes()))

		totalSize += bitmap.GetSizeInBytes()

		out, err := os.Create(fmt.Sprintf("group_%d.roaring", groupID))
		if err != nil {
			panic(err)
		}
		_, err = bitmap.WriteTo(out)
		if err != nil {
			panic(err)
		}
		out.Close()
	}

	fmt.Printf("done, total: %s group members, skipped: %d, total bytes: %s, took: %v\n", humanize.Comma(total), skipped, humanize.Bytes(totalSize), time.Since(start))
}
